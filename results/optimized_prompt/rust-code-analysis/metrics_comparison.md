# Rust Metrics Comparison

Source: `/Users/lithe/Documents/research/code_translation_specs/codenet_statistics copy/r_code/metrics.json`

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
| MI (original) | **74.6749 ✅** | 64.5595 | 67.1011 | +10.1153 | Higher | +7.5738 | Higher | -2.5416 | Lower |
| MI (SEI) | **61.5150 ✅** | 52.0663 | 50.3672 | +9.4488 | Higher | +11.1478 | Higher | +1.6991 | Higher |
| Cyclomatic (sum) | **6 ✅** | 10 | 16 | -4 | Lower | -10 | Lower | -6 | Lower |
| Cognitive (sum) | 5 | 5 | 13 | 0 | Same | -8 | Lower | -8 | Lower |
| Halstead Volume | **1018.6151 ✅** | 2117.5715 | 1272.0008 | -1098.9564 | Lower | -253.3858 | Lower | +845.5706 | Higher |
| Halstead Effort | **23109.8298 ✅** | 41521.0090 | 37688.9140 | -18411.1792 | Lower | -14579.0842 | Lower | +3832.0950 | Higher |
| SLOC | **38 ✅** | 53 | 49 | -15 | Lower | -11 | Lower | +4 | Higher |
| PLOC | **26 ✅** | 34 | 41 | -8 | Lower | -15 | Lower | -7 | Lower |
| LLOC | 17 | **13 ✅** | 18 | +4 | Higher | -1 | Lower | -5 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 7 | -6 | Lower | -1 | Lower | +5 | Higher |
| Blank Lines | 8 | 9 | **5 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

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
| MI (original) | 66.7077 | 65.0302 | **67.9108 ✅** | +1.6774 | Higher | -1.2032 | Lower | -2.8806 | Lower |
| MI (SEI) | **48.2963 ✅** | 42.1443 | 34.0874 | +6.1520 | Higher | +14.2089 | Higher | +8.0568 | Higher |
| Cyclomatic (sum) | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | **1885.0011 ✅** | 2489.9908 | 1929.0164 | -604.9897 | Lower | -44.0152 | Lower | +560.9744 | Higher |
| Halstead Effort | **70687.5422 ✅** | 105851.0982 | 75231.6381 | -35163.5560 | Lower | -4544.0959 | Lower | +30619.4601 | Higher |
| SLOC | 51 | 51 | **47 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| PLOC | 34 | 34 | 36 | 0 | Same | -2 | Lower | -2 | Lower |
| LLOC | 28 | **21 ✅** | 29 | +7 | Higher | -1 | Lower | -8 | Lower |
| Comments (CLOC) | **7 ✅** | 5 | 1 | +2 | Higher | +6 | Higher | +4 | Higher |
| Blank Lines | 11 | 13 | **10 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
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
| MI (original) | **83.2418 ✅** | 72.2137 | 80.4147 | +11.0282 | Higher | +2.8272 | Higher | -8.2010 | Lower |
| MI (SEI) | **89.4593 ✅** | 53.8817 | 76.9953 | +35.5775 | Higher | +12.4639 | Higher | -23.1136 | Lower |
| Cyclomatic (sum) | **6 ✅** | 7 | 8 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 10 | 10 | 12 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | **507.7990 ✅** | 1851.4310 | 645.7255 | -1343.6320 | Lower | -137.9265 | Lower | +1205.7055 | Higher |
| Halstead Effort | **10854.2040 ✅** | 26493.9773 | 14882.4355 | -15639.7733 | Lower | -4028.2314 | Lower | +11611.5419 | Higher |
| SLOC | **28 ✅** | 36 | 30 | -8 | Lower | -2 | Lower | +6 | Higher |
| PLOC | **21 ✅** | 27 | 25 | -6 | Lower | -4 | Lower | +2 | Higher |
| LLOC | 16 | **11 ✅** | 17 | +5 | Higher | -1 | Lower | -6 | Lower |
| Comments (CLOC) | **14 ✅** | 4 | 8 | +10 | Higher | +6 | Higher | -4 | Lower |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
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

## p00015

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.0245 | **65.8937 ✅** | 59.7786 | -6.8692 | Lower | -0.7542 | Lower | +6.1151 | Higher |
| MI (SEI) | 23.9247 | **45.4767 ✅** | 21.4070 | -21.5520 | Lower | +2.5177 | Higher | +24.0697 | Higher |
| Cyclomatic (sum) | 15 | **14 ✅** | 15 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 25 | **19 ✅** | 25 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | 2068.5364 | **1295.0401 ✅** | 1958.3837 | +773.4963 | Higher | +110.1527 | Higher | -663.3435 | Lower |
| Halstead Effort | 98115.7137 | **35451.7240 ✅** | 98870.3997 | +62663.9896 | Higher | -754.6860 | Lower | -63418.6756 | Lower |
| SLOC | 70 | **54 ✅** | 68 | +16 | Higher | +2 | Higher | -14 | Lower |
| PLOC | 56 | **41 ✅** | 54 | +15 | Higher | +2 | Higher | -13 | Lower |
| LLOC | 45 | **21 ✅** | 44 | +24 | Higher | +1 | Higher | -23 | Lower |
| Comments (CLOC) | 2 | **6 ✅** | 1 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | 14 | **9 ✅** | 14 | +5 | Higher | 0 | Same | -5 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
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
| MI (original) | **66.2096 ✅** | 62.5172 | 66.0236 | +3.6924 | Higher | +0.1859 | Higher | -3.5065 | Lower |
| MI (SEI) | 40.8513 | **46.3929 ✅** | 20.5693 | -5.5416 | Lower | +20.2819 | Higher | +25.8236 | Higher |
| Cyclomatic (sum) | **9 ✅** | 12 | 10 | -3 | Lower | -1 | Lower | +2 | Higher |
| Cognitive (sum) | 11 | 21 | 11 | -10 | Lower | 0 | Same | +10 | Higher |
| Halstead Volume | **1357.5171 ✅** | 2557.8586 | 1423.7788 | -1200.3414 | Lower | -66.2616 | Lower | +1134.0798 | Higher |
| Halstead Effort | **33853.0839 ✅** | 47697.9727 | 42103.1719 | -13844.8888 | Lower | -8250.0881 | Lower | +5594.8007 | Higher |
| SLOC | 56 | 55 | 55 | +1 | Higher | +1 | Higher | 0 | Same |
| PLOC | **45 ✅** | 46 | 46 | -1 | Lower | -1 | Lower | 0 | Same |
| LLOC | 19 | 20 | 19 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 4 | **10 ✅** | 0 | -6 | Lower | +4 | Higher | +10 | Higher |
| Blank Lines | 7 | **6 ✅** | 9 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p00024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.5362 | 69.9488 | **89.4368 ✅** | -1.4125 | Lower | -20.9005 | Lower | -19.4880 | Lower |
| MI (SEI) | 49.7403 | 59.7557 | **85.6719 ✅** | -10.0154 | Lower | -35.9315 | Lower | -25.9161 | Lower |
| Cyclomatic (sum) | 8 | 10 | **4 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |
| Cognitive (sum) | 6 | 11 | **4 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| Halstead Volume | 2395.9120 | 1805.0538 | **480.8030 ✅** | +590.8582 | Higher | +1915.1089 | Higher | +1324.2507 | Higher |
| Halstead Effort | 42494.9348 | 26216.6704 | **7692.8484 ✅** | +16278.2644 | Higher | +34802.0864 | Higher | +18523.8220 | Higher |
| SLOC | 41 | 40 | **20 ✅** | +1 | Higher | +21 | Higher | +20 | Higher |
| PLOC | 28 | 26 | **15 ✅** | +2 | Higher | +13 | Higher | +11 | Higher |
| LLOC | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 5 | **9 ✅** | 4 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | 9 | 5 | **2 ✅** | +4 | Higher | +7 | Higher | +3 | Higher |
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
| MI (original) | 71.9432 | 61.7085 | **80.3023 ✅** | +10.2347 | Higher | -8.3591 | Lower | -18.5938 | Lower |
| MI (SEI) | 41.3944 | 48.7310 | **69.3943 ✅** | -7.3365 | Lower | -27.9999 | Lower | -20.6634 | Lower |
| Cyclomatic (sum) | **13 ✅** | 18 | 24 | -5 | Lower | -11 | Lower | -6 | Lower |
| Cognitive (sum) | **15 ✅** | 22 | 24 | -7 | Lower | -9 | Lower | -2 | Lower |
| Halstead Volume | **997.4242 ✅** | 1577.8620 | 2817.7805 | -580.4378 | Lower | -1820.3563 | Lower | -1239.9185 | Lower |
| Halstead Effort | **34909.8467 ✅** | 59643.1844 | 75675.7839 | -24733.3377 | Lower | -40765.9372 | Lower | -16032.5995 | Lower |
| SLOC | 41 | 62 | **15 ✅** | -21 | Lower | +26 | Higher | +47 | Higher |
| PLOC | **36 ✅** | 49 | 50 | -13 | Lower | -14 | Lower | -1 | Lower |
| LLOC | **21 ✅** | 29 | 28 | -8 | Lower | -7 | Lower | +1 | Higher |
| Comments (CLOC) | 1 | **14 ✅** | 2 | -13 | Lower | -1 | Lower | +12 | Higher |
| Blank Lines | 5 | 7 | **-35 ✅** | -2 | Lower | +40 | Higher | +42 | Higher |
| Functions | **2 ✅** | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

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
| MI (original) | 63.3278 | 51.8277 | **66.7390 ✅** | +11.5001 | Higher | -3.4112 | Lower | -14.9113 | Lower |
| MI (SEI) | **38.8110 ✅** | 31.3477 | 21.9068 | +7.4633 | Higher | +16.9042 | Higher | +9.4409 | Higher |
| Cyclomatic (sum) | 19 | 18 | **13 ✅** | +1 | Higher | +6 | Higher | +5 | Higher |
| Cognitive (sum) | 18 | **8 ✅** | 15 | +10 | Higher | +3 | Higher | -7 | Lower |
| Halstead Volume | 2161.0493 | 4096.5535 | **2030.3809 ✅** | -1935.5042 | Lower | +130.6684 | Higher | +2066.1726 | Higher |
| Halstead Effort | 92276.8055 | 139527.1096 | **72452.5410 ✅** | -47250.3042 | Lower | +19824.2645 | Higher | +67074.5687 | Higher |
| SLOC | 50 | 84 | **45 ✅** | -34 | Lower | +5 | Higher | +39 | Higher |
| PLOC | 40 | 66 | **38 ✅** | -26 | Lower | +2 | Higher | +28 | Higher |
| LLOC | 27 | 27 | **26 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 4 | **15 ✅** | 0 | -11 | Lower | +4 | Higher | +15 | Higher |
| Blank Lines | 9 | 10 | **7 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |

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
| MI (original) | **64.9276 ✅** | 60.3718 | 64.4177 | +4.5558 | Higher | +0.5098 | Higher | -4.0459 | Lower |
| MI (SEI) | **57.1438 ✅** | 47.6301 | 52.2329 | +9.5137 | Higher | +4.9109 | Higher | -4.6029 | Lower |
| Cyclomatic (sum) | 18 | 19 | **15 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Cognitive (sum) | 8 | 8 | **7 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | **1306.5873 ✅** | 1857.1728 | 1317.2366 | -550.5855 | Lower | -10.6493 | Lower | +539.9362 | Higher |
| Halstead Effort | 59751.2421 | 76608.3761 | **43109.5615 ✅** | -16857.1340 | Lower | +16641.6805 | Higher | +33498.8146 | Higher |
| SLOC | **54 ✅** | 63 | 58 | -9 | Lower | -4 | Lower | +5 | Higher |
| PLOC | **36 ✅** | 44 | 40 | -8 | Lower | -4 | Lower | +4 | Higher |
| LLOC | **21 ✅** | 25 | 22 | -4 | Lower | -1 | Lower | +3 | Higher |
| Comments (CLOC) | **16 ✅** | 15 | 13 | +1 | Higher | +3 | Higher | +2 | Higher |
| Blank Lines | **9 ✅** | 11 | 10 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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
| MI (original) | 76.9810 | 73.7979 | **77.1207 ✅** | +3.1831 | Higher | -0.1397 | Lower | -3.3228 | Lower |
| MI (SEI) | 54.4197 | 55.4138 | **70.5110 ✅** | -0.9941 | Lower | -16.0912 | Lower | -15.0972 | Lower |
| Cyclomatic (sum) | **7 ✅** | 11 | 8 | -4 | Lower | -1 | Lower | +3 | Higher |
| Cognitive (sum) | **16 ✅** | 19 | 22 | -3 | Lower | -6 | Lower | -3 | Lower |
| Halstead Volume | 884.4812 | 823.7780 | 823.7780 | +60.7032 | Higher | +60.7032 | Higher | 0 | Same |
| Halstead Effort | 17512.7269 | **15350.7855 ✅** | 19822.1573 | +2161.9413 | Higher | -2309.4304 | Lower | -4471.3717 | Lower |
| SLOC | 34 | 40 | 34 | -6 | Lower | 0 | Same | +6 | Higher |
| PLOC | **29 ✅** | 32 | 30 | -3 | Lower | -1 | Lower | +2 | Higher |
| LLOC | 22 | **15 ✅** | 22 | +7 | Higher | 0 | Same | -7 | Lower |
| Comments (CLOC) | 2 | 4 | **8 ✅** | -2 | Lower | -6 | Lower | -4 | Lower |
| Blank Lines | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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
| MI (original) | 79.6197 | **84.1805 ✅** | 77.5102 | -4.5608 | Lower | +2.1095 | Higher | +6.6703 | Higher |
| MI (SEI) | 59.4516 | **81.3282 ✅** | 50.0658 | -21.8766 | Lower | +9.3858 | Higher | +31.2624 | Higher |
| Cyclomatic (sum) | 8 | **6 ✅** | 8 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 752.3839 | **423.9339 ✅** | 764.3204 | +328.4500 | Higher | -11.9366 | Lower | -340.3866 | Lower |
| Halstead Effort | 17272.1168 | **5723.1070 ✅** | 18725.8505 | +11549.0098 | Higher | -1453.7337 | Lower | -13002.7435 | Lower |
| SLOC | 30 | **28 ✅** | 34 | +2 | Higher | -4 | Lower | -6 | Lower |
| PLOC | 25 | **19 ✅** | 26 | +6 | Higher | -1 | Lower | -7 | Lower |
| LLOC | 16 | **9 ✅** | 17 | +7 | Higher | -1 | Lower | -8 | Lower |
| Comments (CLOC) | 2 | **7 ✅** | 1 | -5 | Lower | +1 | Higher | +6 | Higher |
| Blank Lines | 5 | **2 ✅** | 7 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

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
| MI (original) | 55.2429 | **64.8569 ✅** | 48.7744 | -9.6139 | Lower | +6.4685 | Higher | +16.0825 | Higher |
| MI (SEI) | 17.5741 | **52.9198 ✅** | 9.4691 | -35.3457 | Lower | +8.1050 | Higher | +43.4507 | Higher |
| Cyclomatic (sum) | 12 | **9 ✅** | 20 | +3 | Higher | -8 | Lower | -11 | Lower |
| Cognitive (sum) | 12 | **6 ✅** | 31 | +6 | Higher | -19 | Lower | -25 | Lower |
| Halstead Volume | 3632.1564 | **2356.4643 ✅** | 3083.0294 | +1275.6921 | Higher | +549.1271 | Higher | -726.5650 | Lower |
| Halstead Effort | 77091.9107 | **42820.0116 ✅** | 142355.5295 | +34271.8991 | Higher | -65263.6187 | Lower | -99535.5178 | Lower |
| SLOC | 77 | **51 ✅** | 108 | +26 | Higher | -31 | Lower | -57 | Lower |
| PLOC | 59 | **33 ✅** | 95 | +26 | Higher | -36 | Lower | -62 | Lower |
| LLOC | 29 | **18 ✅** | 68 | +11 | Higher | -39 | Lower | -50 | Lower |
| Comments (CLOC) | 2 | **12 ✅** | 3 | -10 | Lower | -1 | Lower | +9 | Higher |
| Blank Lines | 16 | **8 ✅** | 13 | +8 | Higher | +3 | Higher | -5 | Lower |
| Functions | 1 | 1 | 4 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

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
| MI (original) | 65.9959 | **66.9926 ✅** | 60.1287 | -0.9967 | Lower | +5.8672 | Higher | +6.8638 | Higher |
| MI (SEI) | 40.5796 | **45.2783 ✅** | 32.4240 | -4.6988 | Lower | +8.1556 | Higher | +12.8544 | Higher |
| Cyclomatic (sum) | **11 ✅** | 14 | 15 | -3 | Lower | -4 | Lower | -1 | Lower |
| Cognitive (sum) | 16 | **9 ✅** | 37 | +7 | Higher | -21 | Lower | -28 | Lower |
| Halstead Volume | 1225.2439 | **1111.2166 ✅** | 1532.2270 | +114.0273 | Higher | -306.9831 | Lower | -421.0104 | Lower |
| Halstead Effort | 40296.9096 | **36349.6035 ✅** | 64966.4254 | +3947.3061 | Higher | -24669.5158 | Lower | -28616.8219 | Lower |
| SLOC | 57 | **53 ✅** | 72 | +4 | Higher | -15 | Lower | -19 | Lower |
| PLOC | 42 | **37 ✅** | 55 | +5 | Higher | -13 | Lower | -18 | Lower |
| LLOC | 30 | **23 ✅** | 46 | +7 | Higher | -16 | Lower | -23 | Lower |
| Comments (CLOC) | 4 | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Blank Lines | **11 ✅** | 12 | 12 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

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
| MI (original) | 47.8832 | **55.0003 ✅** | 49.8340 | -7.1171 | Lower | -1.9508 | Lower | +5.1663 | Higher |
| MI (SEI) | 13.6444 | **33.1595 ✅** | -0.9547 | -19.5151 | Lower | +14.5991 | Higher | +34.1142 | Higher |
| Cyclomatic (sum) | 29 | **20 ✅** | 28 | +9 | Higher | +1 | Higher | -8 | Lower |
| Cognitive (sum) | 34 | **17 ✅** | 46 | +17 | Higher | -12 | Lower | -29 | Lower |
| Halstead Volume | 3547.2338 | **2566.2586 ✅** | 3341.1629 | +980.9751 | Higher | +206.0709 | Higher | -774.9043 | Lower |
| Halstead Effort | 398892.1575 | **150456.0762 ✅** | 376266.3410 | +248436.0813 | Higher | +22625.8165 | Higher | -225810.2648 | Lower |
| SLOC | 96 | **78 ✅** | 88 | +18 | Higher | +8 | Higher | -10 | Lower |
| PLOC | 76 | **54 ✅** | 73 | +22 | Higher | +3 | Higher | -19 | Lower |
| LLOC | 56 | **33 ✅** | 55 | +23 | Higher | +1 | Higher | -22 | Lower |
| Comments (CLOC) | 5 | **11 ✅** | 0 | -6 | Lower | +5 | Higher | +11 | Higher |
| Blank Lines | 16 | **14 ✅** | 15 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |

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
| MI (original) | **65.8913 ✅** | 62.8395 | 65.4235 | +3.0518 | Higher | +0.4678 | Higher | -2.5840 | Lower |
| MI (SEI) | **43.7899 ✅** | 39.4557 | 39.8191 | +4.3342 | Higher | +3.9708 | Higher | -0.3634 | Lower |
| Cyclomatic (sum) | 13 | 13 | **10 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Cognitive (sum) | 7 | **6 ✅** | 18 | +1 | Higher | -11 | Lower | -12 | Lower |
| Halstead Volume | 1523.1873 | 1583.6313 | **1510.7185 ✅** | -60.4439 | Lower | +12.4688 | Higher | +72.9127 | Higher |
| Halstead Effort | 46577.4648 | 46092.0045 | **40357.7660 ✅** | +485.4603 | Higher | +6219.6988 | Higher | +5734.2385 | Higher |
| SLOC | **52 ✅** | 62 | 56 | -10 | Lower | -4 | Lower | +6 | Higher |
| PLOC | **42 ✅** | 47 | 46 | -5 | Lower | -4 | Lower | +1 | Higher |
| LLOC | 26 | **25 ✅** | 31 | +1 | Higher | -5 | Lower | -6 | Lower |
| Comments (CLOC) | 5 | **6 ✅** | 4 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 9 | 11 | 9 | -2 | Lower | 0 | Same | +2 | Higher |
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
| MI (original) | **71.5188 ✅** | 69.1190 | 68.7674 | +2.3998 | Higher | +2.7514 | Higher | +0.3516 | Higher |
| MI (SEI) | **64.9997 ✅** | 62.8455 | 44.5121 | +2.1542 | Higher | +20.4875 | Higher | +18.3334 | Higher |
| Cyclomatic (sum) | **10 ✅** | 15 | 13 | -5 | Lower | -3 | Lower | +2 | Higher |
| Cognitive (sum) | 11 | **4 ✅** | 9 | +7 | Higher | +2 | Higher | -5 | Lower |
| Halstead Volume | **1065.3932 ✅** | 1261.2061 | 1474.2467 | -195.8129 | Lower | -408.8535 | Lower | -213.0407 | Lower |
| Halstead Effort | **20837.8377 ✅** | 24637.5138 | 39681.8077 | -3799.6761 | Lower | -18843.9701 | Lower | -15044.2939 | Lower |
| SLOC | **43 ✅** | 44 | 44 | -1 | Lower | -1 | Lower | 0 | Same |
| PLOC | 32 | 32 | 36 | 0 | Same | -4 | Lower | -4 | Lower |
| LLOC | 26 | **19 ✅** | 32 | +7 | Higher | -6 | Lower | -13 | Lower |
| Comments (CLOC) | 12 | **13 ✅** | 3 | -1 | Lower | +9 | Higher | +10 | Higher |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00137

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.7484 | 69.9084 | **77.9186 ✅** | +2.8400 | Higher | -5.1702 | Lower | -8.0102 | Lower |
| MI (SEI) | 52.6841 | 57.4757 | **69.0546 ✅** | -4.7915 | Lower | -16.3705 | Lower | -11.5790 | Lower |
| Cyclomatic (sum) | 9 | 7 | **4 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Cognitive (sum) | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 2005.2110 | 1923.4440 | **770.5296 ✅** | +81.7670 | Higher | +1234.6814 | Higher | +1152.9144 | Higher |
| Halstead Effort | 35895.1685 | 31707.9740 | **17420.6691 ✅** | +4187.1945 | Higher | +18474.4994 | Higher | +14287.3049 | Higher |
| SLOC | **33 ✅** | 41 | 35 | -8 | Lower | -2 | Lower | +6 | Higher |
| PLOC | 27 | 27 | **22 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| LLOC | 16 | **14 ✅** | 21 | +2 | Higher | -5 | Lower | -7 | Lower |
| Comments (CLOC) | 3 | **8 ✅** | 7 | -5 | Lower | -4 | Lower | +1 | Higher |
| Blank Lines | **5 ✅** | 6 | 8 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

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
| MI (original) | 53.4897 | **62.4287 ✅** | 53.4811 | -8.9390 | Lower | +0.0086 | Higher | +8.9477 | Higher |
| MI (SEI) | 3.4030 | **46.6406 ✅** | 3.3906 | -43.2376 | Lower | +0.0125 | Higher | +43.2500 | Higher |
| Cyclomatic (sum) | 19 | **8 ✅** | 19 | +11 | Higher | 0 | Same | -11 | Lower |
| Cognitive (sum) | 28 | **2 ✅** | 28 | +26 | Higher | 0 | Same | -26 | Lower |
| Halstead Volume | 3188.6498 | **2778.2854 ✅** | 3193.9495 | +410.3643 | Higher | -5.2997 | Lower | -415.6640 | Lower |
| Halstead Effort | 264028.5918 | **52212.6056 ✅** | 235743.8896 | +211815.9863 | Higher | +28284.7022 | Higher | -183531.2841 | Lower |
| SLOC | 81 | **57 ✅** | 81 | +24 | Higher | 0 | Same | -24 | Lower |
| PLOC | 70 | **33 ✅** | 71 | +37 | Higher | -1 | Lower | -38 | Lower |
| LLOC | 58 | **16 ✅** | 58 | +42 | Higher | 0 | Same | -42 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 0 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 11 | 13 | **10 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
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

## p00156

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.4006 | **56.8233 ✅** | 49.7994 | -7.4227 | Lower | -0.3988 | Lower | +7.0239 | Higher |
| MI (SEI) | 9.9277 | **30.4714 ✅** | -1.3100 | -20.5438 | Lower | +11.2377 | Higher | +31.7815 | Higher |
| Cyclomatic (sum) | 28 | **21 ✅** | 25 | +7 | Higher | +3 | Higher | -4 | Lower |
| Cognitive (sum) | 63 | **26 ✅** | 39 | +37 | Higher | +24 | Higher | -13 | Lower |
| Halstead Volume | 3505.9499 | **2533.4570 ✅** | 3707.8743 | +972.4929 | Higher | -201.9245 | Lower | -1174.4174 | Lower |
| Halstead Effort | 238245.2309 | **109154.2632 ✅** | 337987.0076 | +129090.9677 | Higher | -99741.7768 | Lower | -228832.7444 | Lower |
| SLOC | 89 | **69 ✅** | 89 | +20 | Higher | 0 | Same | -20 | Lower |
| PLOC | 75 | **53 ✅** | 79 | +22 | Higher | -4 | Lower | -26 | Lower |
| LLOC | 65 | **38 ✅** | 77 | +27 | Higher | -12 | Lower | -39 | Lower |
| Comments (CLOC) | 2 | **6 ✅** | 0 | -4 | Lower | +2 | Higher | +6 | Higher |
| Blank Lines | 14 | 13 | **10 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00167

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.9008 | 72.4606 | **76.5241 ✅** | -1.5599 | Lower | -5.6234 | Lower | -4.0635 | Lower |
| MI (SEI) | 43.3757 | 46.3371 | **54.3286 ✅** | -2.9614 | Lower | -10.9529 | Lower | -7.9915 | Lower |
| Cyclomatic (sum) | 9 | 9 | 10 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 11 | 11 | 14 | 0 | Same | -3 | Lower | -3 | Lower |
| Halstead Volume | 1016.4632 | 999.7647 | **928.1232 ✅** | +16.6985 | Higher | +88.3400 | Higher | +71.6415 | Higher |
| Halstead Effort | 27522.6952 | 29992.9414 | **26633.0998 ✅** | -2470.2462 | Lower | +889.5954 | Higher | +3359.8417 | Higher |
| SLOC | 46 | 42 | **33 ✅** | +4 | Higher | +13 | Higher | +9 | Higher |
| PLOC | 39 | 32 | **29 ✅** | +7 | Higher | +10 | Higher | +3 | Higher |
| LLOC | 21 | 21 | **20 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Blank Lines | 7 | 8 | **4 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00187

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.6108 | **63.3084 ✅** | 49.9444 | -12.6976 | Lower | +0.6664 | Higher | +13.3640 | Higher |
| MI (SEI) | 7.8690 | **37.4989 ✅** | -1.5081 | -29.6299 | Lower | +9.3771 | Higher | +39.0070 | Higher |
| Cyclomatic (sum) | 22 | **12 ✅** | 21 | +10 | Higher | +1 | Higher | -9 | Lower |
| Cognitive (sum) | 44 | **8 ✅** | 41 | +36 | Higher | +3 | Higher | -33 | Lower |
| Halstead Volume | 4030.6853 | **2465.5379 ✅** | 3752.9298 | +1565.1474 | Higher | +277.7556 | Higher | -1287.3918 | Lower |
| Halstead Effort | 270727.6980 | **52050.2454 ✅** | 229620.0452 | +218677.4526 | Higher | +41107.6528 | Higher | -177569.7997 | Lower |
| SLOC | 86 | **53 ✅** | 93 | +33 | Higher | -7 | Lower | -40 | Lower |
| PLOC | 76 | **42 ✅** | 82 | +34 | Higher | -6 | Lower | -40 | Lower |
| LLOC | 56 | **13 ✅** | 66 | +43 | Higher | -10 | Lower | -53 | Lower |
| Comments (CLOC) | 1 | **4 ✅** | 0 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 10 | **8 ✅** | 11 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p00190

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.5794 | **58.7633 ✅** | 32.8490 | -21.1838 | Lower | +4.7305 | Higher | +25.9143 | Higher |
| MI (SEI) | -18.2269 | **31.1616 ✅** | -17.0972 | -49.3886 | Lower | -1.1298 | Lower | +48.2588 | Higher |
| Cyclomatic (sum) | 32 | **18 ✅** | 33 | +14 | Higher | -1 | Lower | -15 | Lower |
| Cognitive (sum) | 61 | **13 ✅** | 60 | +48 | Higher | +1 | Higher | -47 | Lower |
| Halstead Volume | 4867.1526 | **1992.1421 ✅** | 6078.9183 | +2875.0105 | Higher | -1211.7658 | Lower | -4086.7763 | Lower |
| Halstead Effort | 303517.0673 | **54731.4822 ✅** | 624755.9307 | +248785.5850 | Higher | -321238.8635 | Lower | -570024.4485 | Lower |
| SLOC | 157 | **69 ✅** | 193 | +88 | Higher | -36 | Lower | -124 | Lower |
| PLOC | 138 | **53 ✅** | 174 | +85 | Higher | -36 | Lower | -121 | Lower |
| LLOC | 56 | **19 ✅** | 136 | +37 | Higher | -80 | Lower | -117 | Lower |
| Comments (CLOC) | 0 | **5 ✅** | 2 | -5 | Lower | -2 | Lower | +3 | Higher |
| Blank Lines | 19 | **13 ✅** | 17 | +6 | Higher | +2 | Higher | -4 | Lower |
| Functions | 6 | **4 ✅** | 5 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 8 | **5 ✅** | 8 | +3 | Higher | 0 | Same | -3 | Lower |

## p00203

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.1414 | **60.5640 ✅** | 49.8152 | -9.4226 | Lower | +1.3262 | Higher | +10.7488 | Higher |
| MI (SEI) | 9.1436 | **45.3968 ✅** | -0.7781 | -36.2531 | Lower | +9.9217 | Higher | +46.1748 | Higher |
| Cyclomatic (sum) | 27 | **14 ✅** | 30 | +13 | Higher | -3 | Lower | -16 | Lower |
| Cognitive (sum) | 44 | **8 ✅** | 39 | +36 | Higher | +5 | Higher | -31 | Lower |
| Halstead Volume | 2917.5387 | **1410.9640 ✅** | 3297.2197 | +1506.5747 | Higher | -379.6810 | Lower | -1886.2557 | Lower |
| Halstead Effort | 187087.1702 | **45673.4288 ✅** | 257424.3995 | +141413.7414 | Higher | -70337.2293 | Lower | -211750.9707 | Lower |
| SLOC | 86 | **73 ✅** | 86 | +13 | Higher | 0 | Same | -13 | Lower |
| PLOC | 76 | **46 ✅** | 77 | +30 | Higher | -1 | Lower | -31 | Lower |
| LLOC | 45 | **19 ✅** | 50 | +26 | Higher | -5 | Lower | -31 | Lower |
| Comments (CLOC) | 1 | **15 ✅** | 0 | -14 | Lower | +1 | Higher | +15 | Higher |
| Blank Lines | 10 | 14 | **9 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Functions | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | **1 ✅** | 3 | 5 | -2 | Lower | -4 | Lower | -2 | Lower |
| Exits | 6 | 3 | 3 | +3 | Higher | +3 | Higher | 0 | Same |

## p00208

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.6758 ✅** | 70.0046 | 72.4648 | +2.6712 | Higher | +0.2109 | Higher | -2.4602 | Lower |
| MI (SEI) | 59.0313 | 55.5049 | **71.4406 ✅** | +3.5264 | Higher | -12.4093 | Lower | -15.9357 | Lower |
| Cyclomatic (sum) | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 15 | 13 | **8 ✅** | +2 | Higher | +7 | Higher | +5 | Higher |
| Halstead Volume | **829.8336 ✅** | 991.8765 | 998.9625 | -162.0429 | Lower | -169.1289 | Lower | -7.0860 | Lower |
| Halstead Effort | **21972.5507 ✅** | 25959.8025 | 32466.2827 | -3987.2518 | Lower | -10493.7320 | Lower | -6506.4803 | Lower |
| SLOC | 44 | 49 | **42 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| PLOC | **32 ✅** | 34 | 36 | -2 | Lower | -4 | Lower | -2 | Lower |
| LLOC | 27 | **26 ✅** | 33 | +1 | Higher | -6 | Lower | -7 | Lower |
| Comments (CLOC) | 7 | 8 | **17 ✅** | -1 | Lower | -10 | Lower | -9 | Lower |
| Blank Lines | 8 | 9 | **6 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00244

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.2621 | **45.3676 ✅** | 33.7113 | -3.1055 | Lower | +8.5508 | Higher | +11.6563 | Higher |
| MI (SEI) | -4.4358 | **18.2062 ✅** | -15.3281 | -22.6420 | Lower | +10.8924 | Higher | +33.5344 | Higher |
| Cyclomatic (sum) | 33 | **27 ✅** | 35 | +6 | Higher | -2 | Lower | -8 | Lower |
| Cognitive (sum) | 37 | **23 ✅** | 42 | +14 | Higher | -5 | Lower | -19 | Lower |
| Halstead Volume | **3946.7127 ✅** | 4483.6382 | 6065.3199 | -536.9255 | Lower | -2118.6072 | Lower | -1581.6817 | Lower |
| Halstead Effort | 196151.6221 | **139889.5134 ✅** | 473281.5776 | +56262.1088 | Higher | -277129.9554 | Lower | -333392.0642 | Lower |
| SLOC | 124 | **107 ✅** | 178 | +17 | Higher | -54 | Lower | -71 | Lower |
| PLOC | 102 | **80 ✅** | 149 | +22 | Higher | -47 | Lower | -69 | Lower |
| LLOC | 48 | **30 ✅** | 95 | +18 | Higher | -47 | Lower | -65 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 2 | -12 | Lower | -1 | Lower | +11 | Higher |
| Blank Lines | 22 | **17 ✅** | 27 | +5 | Higher | -5 | Lower | -10 | Lower |
| Functions | 5 | 5 | 9 | 0 | Same | -4 | Lower | -4 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **4 ✅** | 5 | 9 | -1 | Lower | -5 | Lower | -4 | Lower |

## p00252

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.8405 | **87.5403 ✅** | 79.1434 | -13.6998 | Lower | -5.3029 | Lower | +8.3968 | Higher |
| MI (SEI) | 63.0460 | **87.1928 ✅** | 65.3240 | -24.1468 | Lower | -2.2780 | Lower | +21.8688 | Higher |
| Cyclomatic (sum) | 10 | **9 ✅** | 17 | +1 | Higher | -7 | Lower | -8 | Lower |
| Cognitive (sum) | 4 | **3 ✅** | 6 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | 1088.7840 | **651.1952 ✅** | 864.8687 | +437.5887 | Higher | +223.9153 | Higher | -213.6734 | Lower |
| Halstead Effort | 26747.7933 | **16849.6769 ✅** | 18464.9464 | +9898.1163 | Higher | +8282.8469 | Higher | -1615.2695 | Lower |
| SLOC | 37 | **19 ✅** | 26 | +18 | Higher | +11 | Higher | -7 | Lower |
| PLOC | 24 | **14 ✅** | 20 | +10 | Higher | +4 | Higher | -6 | Lower |
| LLOC | 12 | **5 ✅** | 12 | +7 | Higher | 0 | Same | -7 | Lower |
| Comments (CLOC) | **7 ✅** | 5 | 3 | +2 | Higher | +4 | Higher | +2 | Higher |
| Blank Lines | 7 | **2 ✅** | 3 | +5 | Higher | +4 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |

## p00256

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.5435 | **59.7173 ✅** | 47.6488 | -12.1738 | Lower | -0.1054 | Lower | +12.0684 | Higher |
| MI (SEI) | 3.1504 | **43.7781 ✅** | 6.3526 | -40.6277 | Lower | -3.2022 | Lower | +37.4255 | Higher |
| Cyclomatic (sum) | 25 | **20 ✅** | 24 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 51 | **18 ✅** | 50 | +33 | Higher | +1 | Higher | -32 | Lower |
| Halstead Volume | 3979.8276 | **2015.1522 ✅** | 4076.3726 | +1964.6754 | Higher | -96.5451 | Lower | -2061.2205 | Lower |
| Halstead Effort | 320532.1898 | **78651.9994 ✅** | 333493.4292 | +241880.1905 | Higher | -12961.2394 | Lower | -254841.4298 | Lower |
| SLOC | 100 | **63 ✅** | 100 | +37 | Higher | 0 | Same | -37 | Lower |
| PLOC | 89 | **46 ✅** | 90 | +43 | Higher | -1 | Lower | -44 | Lower |
| LLOC | 106 | **18 ✅** | 106 | +88 | Higher | 0 | Same | -88 | Lower |
| Comments (CLOC) | 1 | **12 ✅** | 2 | -11 | Lower | -1 | Lower | +10 | Higher |
| Blank Lines | 11 | **6 ✅** | 10 | +5 | Higher | +1 | Higher | -4 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 5 | 0 | -5 | Lower | 0 | Same | +5 | Higher |

## p00261

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.9858 | 67.4370 | **70.3395 ✅** | +0.5488 | Higher | -2.3537 | Lower | -2.9025 | Lower |
| MI (SEI) | 33.8581 | **56.4573 ✅** | 26.4904 | -22.5992 | Lower | +7.3678 | Higher | +29.9670 | Higher |
| Cyclomatic (sum) | 9 | 9 | **7 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 17 | 13 | **11 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Halstead Volume | 1145.2226 | **1072.0884 ✅** | 1083.4332 | +73.1342 | Higher | +61.7894 | Higher | -11.3449 | Lower |
| Halstead Effort | **18082.4623 ✅** | 22479.2727 | 22836.7413 | -4396.8104 | Lower | -4754.2790 | Lower | -357.4686 | Lower |
| SLOC | 53 | 56 | **48 ✅** | -3 | Lower | +5 | Higher | +8 | Higher |
| PLOC | 46 | **40 ✅** | 41 | +6 | Higher | +5 | Higher | -1 | Lower |
| LLOC | 23 | 19 | 19 | +4 | Higher | +4 | Higher | 0 | Same |
| Comments (CLOC) | 1 | **13 ✅** | 0 | -12 | Lower | +1 | Higher | +13 | Higher |
| Blank Lines | 7 | 9 | 7 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p00270

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.1656 | **43.7617 ✅** | 23.7427 | -4.5961 | Lower | +15.4229 | Higher | +20.0190 | Higher |
| MI (SEI) | -9.8292 | **7.1948 ✅** | -30.2440 | -17.0240 | Lower | +20.4148 | Higher | +37.4388 | Higher |
| Cyclomatic (sum) | 29 | **26 ✅** | 43 | +3 | Higher | -14 | Lower | -17 | Lower |
| Cognitive (sum) | 36 | **21 ✅** | 63 | +15 | Higher | -27 | Lower | -42 | Lower |
| Halstead Volume | 5248.2151 | **4705.0304 ✅** | 9334.1341 | +543.1847 | Higher | -4085.9190 | Lower | -4629.1037 | Lower |
| Halstead Effort | 323242.3399 | **119521.8166 ✅** | 1118981.5711 | +203720.5233 | Higher | -795739.2311 | Lower | -999459.7545 | Lower |
| SLOC | 145 | **118 ✅** | 256 | +27 | Higher | -111 | Lower | -138 | Lower |
| PLOC | 122 | **99 ✅** | 227 | +23 | Higher | -105 | Lower | -128 | Lower |
| LLOC | 77 | **34 ✅** | 175 | +43 | Higher | -98 | Lower | -141 | Lower |
| Comments (CLOC) | 1 | **6 ✅** | 2 | -5 | Lower | -1 | Lower | +4 | Higher |
| Blank Lines | 23 | **19 ✅** | 27 | +4 | Higher | -4 | Lower | -8 | Lower |
| Functions | **5 ✅** | 7 | 8 | -2 | Lower | -3 | Lower | -1 | Lower |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |

## p00271

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.6381 | 76.8551 | **89.3166 ✅** | +3.7830 | Higher | -8.6785 | Lower | -12.4615 | Lower |
| MI (SEI) | 68.0460 | 65.3449 | **81.6933 ✅** | +2.7011 | Higher | -13.6473 | Lower | -16.3484 | Lower |
| Cyclomatic (sum) | 6 | 6 | **3 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Cognitive (sum) | 3 | 2 | **1 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Halstead Volume | 675.7805 | 1398.7942 | **514.2969 ✅** | -723.0138 | Lower | +161.4836 | Higher | +884.4974 | Higher |
| Halstead Effort | 12579.9132 | 18806.0115 | **5534.2818 ✅** | -6226.0983 | Lower | +7045.6314 | Higher | +13271.7297 | Higher |
| SLOC | 30 | 30 | **20 ✅** | 0 | Same | +10 | Higher | +10 | Higher |
| PLOC | 22 | 20 | **13 ✅** | +2 | Higher | +9 | Higher | +7 | Higher |
| LLOC | 13 | 9 | 9 | +4 | Higher | +4 | Higher | 0 | Same |
| Comments (CLOC) | 4 | **5 ✅** | 3 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00283

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.5809 ✅** | 52.9229 | 58.3916 | +6.6580 | Higher | +1.1894 | Higher | -5.4687 | Lower |
| MI (SEI) | 11.6817 | **16.0437 ✅** | 10.1695 | -4.3619 | Lower | +1.5123 | Higher | +5.8742 | Higher |
| Cyclomatic (sum) | **14 ✅** | 17 | 16 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | 21 | **13 ✅** | 20 | +8 | Higher | +1 | Higher | -7 | Lower |
| Halstead Volume | 2447.1795 | **2443.2903 ✅** | 2562 | +3.8892 | Higher | -114.8205 | Lower | -118.7097 | Lower |
| Halstead Effort | 136893.7386 | **98220.2701 ✅** | 144468.3333 | +38673.4685 | Higher | -7574.5948 | Lower | -46248.0633 | Lower |
| SLOC | **65 ✅** | 94 | 67 | -29 | Lower | -2 | Lower | +27 | Higher |
| PLOC | **55 ✅** | 81 | 57 | -26 | Lower | -2 | Lower | +24 | Higher |
| LLOC | 42 | **30 ✅** | 46 | +12 | Higher | -4 | Lower | -16 | Lower |
| Comments (CLOC) | 0 | **3 ✅** | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 10 | 13 | 10 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |
| Exits | 3 | 6 | 3 | -3 | Lower | 0 | Same | +3 | Higher |

## p00288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 19.7446 | **46.5718 ✅** | 19.4832 | -26.8273 | Lower | +0.2613 | Higher | +27.0886 | Higher |
| MI (SEI) | -35.3814 | **13.0044 ✅** | -35.4837 | -48.3858 | Lower | +0.1023 | Higher | +48.4881 | Higher |
| Cyclomatic (sum) | 52 | **17 ✅** | 54 | +35 | Higher | -2 | Lower | -37 | Lower |
| Cognitive (sum) | 81 | **18 ✅** | 87 | +63 | Higher | -6 | Lower | -69 | Lower |
| Halstead Volume | 10344.3466 | **4304.0437 ✅** | 10654.3064 | +6040.3030 | Higher | -309.9598 | Lower | -6350.2628 | Lower |
| Halstead Effort | 1170694.6753 | **120619.0596 ✅** | 1206067.4894 | +1050075.6157 | Higher | -35372.8141 | Lower | -1085448.4298 | Lower |
| SLOC | 279 | **116 ✅** | 273 | +163 | Higher | +6 | Higher | -157 | Lower |
| PLOC | 242 | **91 ✅** | 244 | +151 | Higher | -2 | Lower | -153 | Lower |
| LLOC | 161 | **50 ✅** | 176 | +111 | Higher | -15 | Lower | -126 | Lower |
| Comments (CLOC) | 2 | **8 ✅** | 2 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | 35 | **20 ✅** | 27 | +15 | Higher | +8 | Higher | -7 | Lower |
| Functions | 14 | **5 ✅** | 13 | +9 | Higher | +1 | Higher | -8 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | **4 ✅** | 8 | +5 | Higher | +1 | Higher | -4 | Lower |

## p00292

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.2551 | 75.9488 | **84.6390 ✅** | -5.6937 | Lower | -14.3840 | Lower | -8.6903 | Lower |
| MI (SEI) | 56.7774 | 65.3304 | **78.6484 ✅** | -8.5530 | Lower | -21.8710 | Lower | -13.3180 | Lower |
| Cyclomatic (sum) | 6 | 6 | **3 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Cognitive (sum) | 2 | 4 | **1 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Halstead Volume | 2197.8510 | 797.2888 | **630.9018 ✅** | +1400.5622 | Higher | +1566.9492 | Higher | +166.3870 | Higher |
| Halstead Effort | 43468.6080 | 11866.3149 | **8766.2146 ✅** | +31602.2930 | Higher | +34702.3933 | Higher | +3100.1003 | Higher |
| SLOC | 39 | 38 | **25 ✅** | +1 | Higher | +14 | Higher | +13 | Higher |
| PLOC | 25 | 24 | **15 ✅** | +1 | Higher | +10 | Higher | +9 | Higher |
| LLOC | 13 | 14 | **11 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Comments (CLOC) | 7 | 7 | 5 | 0 | Same | +2 | Higher | +2 | Higher |
| Blank Lines | 8 | 7 | **5 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p00315

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.5286 | **58.2278 ✅** | 53.5197 | -4.6992 | Lower | +0.0089 | Higher | +4.7082 | Higher |
| MI (SEI) | 32.3330 | **41.2102 ✅** | 22.0160 | -8.8772 | Lower | +10.3170 | Higher | +19.1942 | Higher |
| Cyclomatic (sum) | 17 | **15 ✅** | 17 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 16 | 19 | 16 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 3457.6199 | **2115.5379 ✅** | 3463.5506 | +1342.0819 | Higher | -5.9307 | Lower | -1348.0127 | Lower |
| Halstead Effort | 268410.0909 | **83482.3819 ✅** | 268870.4855 | +184927.7090 | Higher | -460.3947 | Lower | -185388.1037 | Lower |
| SLOC | 81 | **73 ✅** | 81 | +8 | Higher | 0 | Same | -8 | Lower |
| PLOC | 66 | **48 ✅** | 66 | +18 | Higher | 0 | Same | -18 | Lower |
| LLOC | 45 | **37 ✅** | 45 | +8 | Higher | 0 | Same | -8 | Lower |
| Comments (CLOC) | 13 | **14 ✅** | 5 | -1 | Lower | +8 | Higher | +9 | Higher |
| Blank Lines | 15 | **13 ✅** | 15 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p00317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.4404 | **56.9383 ✅** | 31.2468 | -12.4979 | Lower | +13.1936 | Higher | +25.6915 | Higher |
| MI (SEI) | -8.9396 | **38.9327 ✅** | -14.9927 | -47.8723 | Lower | +6.0531 | Higher | +53.9254 | Higher |
| Cyclomatic (sum) | 26 | **17 ✅** | 42 | +9 | Higher | -16 | Lower | -25 | Lower |
| Cognitive (sum) | 54 | **27 ✅** | 72 | +27 | Higher | -18 | Lower | -45 | Lower |
| Halstead Volume | 3818.6486 | **1727.4339 ✅** | 6557.4861 | +2091.2147 | Higher | -2738.8374 | Lower | -4830.0522 | Lower |
| Halstead Effort | 233441.9156 | **55709.7432 ✅** | 611435.8971 | +177732.1724 | Higher | -377993.9815 | Lower | -555726.1539 | Lower |
| SLOC | 121 | **82 ✅** | 183 | +39 | Higher | -62 | Lower | -101 | Lower |
| PLOC | 108 | **59 ✅** | 162 | +49 | Higher | -54 | Lower | -103 | Lower |
| LLOC | 86 | **32 ✅** | 124 | +54 | Higher | -38 | Lower | -92 | Lower |
| Comments (CLOC) | 0 | **15 ✅** | 4 | -15 | Lower | -4 | Lower | +11 | Higher |
| Blank Lines | 13 | **11 ✅** | 17 | +2 | Higher | -4 | Lower | -6 | Lower |
| Functions | **2 ✅** | 3 | 8 | -1 | Lower | -6 | Lower | -5 | Lower |
| Closures | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |
| Exits | **2 ✅** | 5 | 11 | -3 | Lower | -9 | Lower | -6 | Lower |

## p00320

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.3183 | **58.3536 ✅** | 53.8697 | -3.0353 | Lower | +1.4486 | Higher | +4.4839 | Higher |
| MI (SEI) | 5.7358 | **37.3677 ✅** | 3.7477 | -31.6320 | Lower | +1.9881 | Higher | +33.6201 | Higher |
| Cyclomatic (sum) | 16 | 16 | 17 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 17 | **10 ✅** | 17 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 2881.1864 | **2836.3013 ✅** | 3001.1192 | +44.8851 | Higher | -119.9328 | Lower | -164.8179 | Lower |
| Halstead Effort | 222859.7651 | **65541.5561 ✅** | 275888.5990 | +157318.2090 | Higher | -53028.8339 | Lower | -210347.0429 | Lower |
| SLOC | 78 | **65 ✅** | 83 | +13 | Higher | -5 | Lower | -18 | Lower |
| PLOC | 70 | **49 ✅** | 74 | +21 | Higher | -4 | Lower | -25 | Lower |
| LLOC | 44 | **19 ✅** | 46 | +25 | Higher | -2 | Lower | -27 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 0 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 8 | 8 | 9 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00329

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.0619 | **56.9652 ✅** | 47.2015 | -3.9033 | Lower | +5.8605 | Higher | +9.7638 | Higher |
| MI (SEI) | 14.1287 | **35.3373 ✅** | 7.5949 | -21.2086 | Lower | +6.5338 | Higher | +27.7424 | Higher |
| Cyclomatic (sum) | 18 | **17 ✅** | 25 | +1 | Higher | -7 | Lower | -8 | Lower |
| Cognitive (sum) | 21 | **16 ✅** | 25 | +5 | Higher | -4 | Lower | -9 | Lower |
| Halstead Volume | 2606.0940 | **2468.6166 ✅** | 3158.4604 | +137.4774 | Higher | -552.3664 | Lower | -689.8438 | Lower |
| Halstead Effort | 151479.2165 | **97232.6367 ✅** | 195555.7407 | +54246.5798 | Higher | -44076.5242 | Lower | -98323.1040 | Lower |
| SLOC | 90 | **73 ✅** | 110 | +17 | Higher | -20 | Lower | -37 | Lower |
| PLOC | 78 | **61 ✅** | 98 | +17 | Higher | -20 | Lower | -37 | Lower |
| LLOC | 52 | **37 ✅** | 77 | +15 | Higher | -25 | Lower | -40 | Lower |
| Comments (CLOC) | 2 | **10 ✅** | 3 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |
| Functions | **4 ✅** | 5 | 8 | -1 | Lower | -4 | Lower | -3 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **4 ✅** | 6 | 5 | -2 | Lower | -1 | Lower | +1 | Higher |

## p00336

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.4504 ✅** | 67.9834 | 73.8086 | +6.4670 | Higher | +0.6418 | Higher | -5.8252 | Lower |
| MI (SEI) | **68.0026 ✅** | 57.3287 | 61.5759 | +10.6740 | Higher | +6.4267 | Higher | -4.2472 | Lower |
| Cyclomatic (sum) | 6 | 8 | 6 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 7 | 8 | 7 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 1063.5479 | 1350.0372 | **1025.5640 ✅** | -286.4893 | Lower | +37.9839 | Higher | +324.4732 | Higher |
| Halstead Effort | 42339.3351 | 50457.6415 | **32996.4082 ✅** | -8118.3064 | Lower | +9342.9269 | Higher | +17461.2333 | Higher |
| SLOC | **38 ✅** | 51 | 40 | -13 | Lower | -2 | Lower | +11 | Higher |
| PLOC | **23 ✅** | 29 | 24 | -6 | Lower | -1 | Lower | +5 | Higher |
| LLOC | **18 ✅** | 20 | 19 | -2 | Lower | -1 | Lower | +1 | Higher |
| Comments (CLOC) | 10 | **12 ✅** | 7 | -2 | Lower | +3 | Higher | +5 | Higher |
| Blank Lines | **8 ✅** | 11 | 9 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00378

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.4410 | **78.8432 ✅** | 78.4008 | -5.4021 | Lower | -4.9597 | Lower | +0.4424 | Higher |
| MI (SEI) | 54.7971 | **79.4213 ✅** | 60.8561 | -24.6242 | Lower | -6.0590 | Lower | +18.5652 | Higher |
| Cyclomatic (sum) | 10 | **3 ✅** | 6 | +7 | Higher | +4 | Higher | -3 | Lower |
| Cognitive (sum) | 11 | **0 ✅** | 9 | +11 | Higher | +2 | Higher | -9 | Lower |
| Halstead Volume | 922.1952 | 983.9595 | **849.8521 ✅** | -61.7643 | Lower | +72.3431 | Higher | +134.1074 | Higher |
| Halstead Effort | 32018.6171 | **11357.9467 ✅** | 22170.0546 | +20660.6704 | Higher | +9848.5625 | Higher | -10812.1079 | Lower |
| SLOC | 40 | **31 ✅** | 32 | +9 | Higher | +8 | Higher | -1 | Lower |
| PLOC | 28 | **17 ✅** | 24 | +11 | Higher | +4 | Higher | -7 | Lower |
| LLOC | 20 | **7 ✅** | 20 | +13 | Higher | 0 | Same | -13 | Lower |
| Comments (CLOC) | 4 | **12 ✅** | 3 | -8 | Lower | +1 | Higher | +9 | Higher |
| Blank Lines | 8 | 5 | 5 | +3 | Higher | +3 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00428

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.2719 | **65.4372 ✅** | 61.1037 | -2.1653 | Lower | +2.1682 | Higher | +4.3336 | Higher |
| MI (SEI) | 31.7425 | **46.4937 ✅** | 24.3212 | -14.7512 | Lower | +7.4213 | Higher | +22.1725 | Higher |
| Cyclomatic (sum) | 17 | **13 ✅** | 19 | +4 | Higher | -2 | Lower | -6 | Lower |
| Cognitive (sum) | 29 | **11 ✅** | 27 | +18 | Higher | +2 | Higher | -16 | Lower |
| Halstead Volume | 1676.5533 | **1477.8035 ✅** | 1878.1906 | +198.7498 | Higher | -201.6373 | Lower | -400.3871 | Lower |
| Halstead Effort | 53020.9971 | **47018.7806 ✅** | 87426.7424 | +6002.2165 | Higher | -34405.7452 | Lower | -40407.9617 | Lower |
| SLOC | 56 | **54 ✅** | 60 | +2 | Higher | -4 | Lower | -6 | Lower |
| PLOC | 47 | **37 ✅** | 54 | +10 | Higher | -7 | Lower | -17 | Lower |
| LLOC | 33 | **22 ✅** | 43 | +11 | Higher | -10 | Lower | -21 | Lower |
| Comments (CLOC) | 2 | **7 ✅** | 1 | -5 | Lower | +1 | Higher | +6 | Higher |
| Blank Lines | 8 | 11 | **6 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 3 | 5 | **2 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.8895 | 79.2759 | **82.7071 ✅** | +1.6136 | Higher | -1.8176 | Lower | -3.4312 | Lower |
| MI (SEI) | **79.1635 ✅** | 74.5411 | 76.5183 | +4.6224 | Higher | +2.6452 | Higher | -1.9772 | Lower |
| Cyclomatic (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **500.1075 ✅** | 567.8304 | 527.3324 | -67.7229 | Lower | -27.2248 | Lower | +40.4981 | Higher |
| Halstead Effort | 8182.3148 | 7787.3886 | **6403.3214 ✅** | +394.9262 | Higher | +1778.9934 | Higher | +1384.0672 | Higher |
| SLOC | 33 | 35 | **29 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |
| PLOC | **17 ✅** | 20 | 18 | -3 | Lower | -1 | Lower | +2 | Higher |
| LLOC | 11 | 11 | **9 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Comments (CLOC) | **10 ✅** | 9 | 6 | +1 | Higher | +4 | Higher | +3 | Higher |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00445

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.2214 | 76.2347 | **82.9515 ✅** | +2.9867 | Higher | -3.7301 | Lower | -6.7168 | Lower |
| MI (SEI) | 67.6609 | **70.2158 ✅** | 69.2713 | -2.5549 | Lower | -1.6104 | Lower | +0.9445 | Higher |
| Cyclomatic (sum) | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 11 | **4 ✅** | 9 | +7 | Higher | +2 | Higher | -5 | Lower |
| Halstead Volume | 630.9011 | 695.9688 | **601.3779 ✅** | -65.0677 | Lower | +29.5233 | Higher | +94.5909 | Higher |
| Halstead Effort | 12904.7960 | 14673.3423 | **11070.8200 ✅** | -1768.5463 | Lower | +1833.9760 | Higher | +3602.5224 | Higher |
| SLOC | 33 | 39 | **27 ✅** | -6 | Lower | +6 | Higher | +12 | Higher |
| PLOC | 23 | 23 | **21 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| LLOC | 17 | 16 | 16 | +1 | Higher | +1 | Higher | 0 | Same |
| Comments (CLOC) | 5 | **10 ✅** | 3 | -5 | Lower | +2 | Higher | +7 | Higher |
| Blank Lines | 6 | 8 | **4 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00450

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.3916 | 72.0693 | **75.5163 ✅** | +0.3223 | Higher | -3.1247 | Lower | -3.4470 | Lower |
| MI (SEI) | 60.2444 | 56.7396 | **62.3538 ✅** | +3.5048 | Higher | -2.1094 | Lower | -5.6142 | Lower |
| Cyclomatic (sum) | 13 | 11 | 11 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | 19 | **9 ✅** | 20 | +10 | Higher | -1 | Lower | -11 | Lower |
| Halstead Volume | 988.1889 | **916.9113 ✅** | 982.1539 | +71.2776 | Higher | +6.0350 | Higher | -65.2426 | Lower |
| Halstead Effort | 35776.9311 | **31655.2705 ✅** | 37218.4644 | +4121.6606 | Higher | -1441.5332 | Lower | -5563.1938 | Lower |
| SLOC | 40 | 43 | **34 ✅** | -3 | Lower | +6 | Higher | +9 | Higher |
| PLOC | 30 | 33 | **26 ✅** | -3 | Lower | +4 | Higher | +7 | Higher |
| LLOC | **16 ✅** | 22 | 21 | -6 | Lower | -5 | Lower | +1 | Higher |
| Comments (CLOC) | **7 ✅** | 6 | 5 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 6 | 8 | **5 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00453

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.2540 | **58.8862 ✅** | 49.8038 | -4.6322 | Lower | +4.4502 | Higher | +9.0823 | Higher |
| MI (SEI) | 13.6652 | **41.4557 ✅** | -1.2018 | -27.7905 | Lower | +14.8670 | Higher | +42.6575 | Higher |
| Cyclomatic (sum) | 21 | **13 ✅** | 26 | +8 | Higher | -5 | Lower | -13 | Lower |
| Cognitive (sum) | 64 | **16 ✅** | 65 | +48 | Higher | -1 | Lower | -49 | Lower |
| Halstead Volume | 3339.2291 | **2923.4803 ✅** | 4405.3265 | +415.7488 | Higher | -1066.0974 | Lower | -1481.8462 | Lower |
| Halstead Effort | 267644.2702 | **60296.7809 ✅** | 381777.8842 | +207347.4893 | Higher | -114133.6140 | Lower | -321481.1032 | Lower |
| SLOC | 74 | **65 ✅** | 83 | +9 | Higher | -9 | Lower | -18 | Lower |
| PLOC | 66 | **46 ✅** | 76 | +20 | Higher | -10 | Lower | -30 | Lower |
| LLOC | 52 | **27 ✅** | 61 | +25 | Higher | -9 | Lower | -34 | Lower |
| Comments (CLOC) | 1 | **12 ✅** | 0 | -11 | Lower | +1 | Higher | +12 | Higher |
| Blank Lines | 8 | 10 | **7 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 4 | 0 | Same | -4 | Lower | -4 | Lower |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

## p00465

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.3549 | **49.7308 ✅** | 35.1946 | -8.3759 | Lower | +6.1603 | Higher | +14.5362 | Higher |
| MI (SEI) | 3.8941 | **18.2918 ✅** | -9.3164 | -14.3977 | Lower | +13.2105 | Higher | +27.6083 | Higher |
| Cyclomatic (sum) | 31 | **19 ✅** | 36 | +12 | Higher | -5 | Lower | -17 | Lower |
| Cognitive (sum) | 41 | **16 ✅** | 49 | +25 | Higher | -8 | Lower | -33 | Lower |
| Halstead Volume | 5264.8059 | **3869.6489 ✅** | 5525.5184 | +1395.1569 | Higher | -260.7125 | Lower | -1655.8695 | Lower |
| Halstead Effort | 342797.3596 | **160123.4042 ✅** | 479062.4455 | +182673.9554 | Higher | -136265.0859 | Lower | -318939.0413 | Lower |
| SLOC | 123 | **96 ✅** | 165 | +27 | Higher | -42 | Lower | -69 | Lower |
| PLOC | 99 | **71 ✅** | 138 | +28 | Higher | -39 | Lower | -67 | Lower |
| LLOC | 70 | **46 ✅** | 98 | +24 | Higher | -28 | Lower | -52 | Lower |
| Comments (CLOC) | 6 | **7 ✅** | 4 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 19 | **18 ✅** | 25 | +1 | Higher | -6 | Lower | -7 | Lower |
| Functions | 3 | **2 ✅** | 9 | +1 | Higher | -6 | Lower | -7 | Lower |
| Closures | 4 | 6 | **0 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |
| Exits | 2 | **1 ✅** | 6 | +1 | Higher | -4 | Lower | -5 | Lower |

## p00468

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.1415 | **65.7453 ✅** | 61.7736 | -4.6038 | Lower | -0.6321 | Lower | +3.9718 | Higher |
| MI (SEI) | 38.4754 | **49.7242 ✅** | 24.4671 | -11.2487 | Lower | +14.0083 | Higher | +25.2571 | Higher |
| Cyclomatic (sum) | 16 | **10 ✅** | 14 | +6 | Higher | +2 | Higher | -4 | Lower |
| Cognitive (sum) | 18 | **10 ✅** | 18 | +8 | Higher | 0 | Same | -8 | Lower |
| Halstead Volume | 1659.2502 | **1590.4227 ✅** | 1684.6864 | +68.8276 | Higher | -25.4362 | Lower | -94.2637 | Lower |
| Halstead Effort | **54202.1744 ✅** | 54869.5825 | 69979.2811 | -667.4081 | Lower | -15777.1067 | Lower | -15109.6986 | Lower |
| SLOC | 65 | **54 ✅** | 64 | +11 | Higher | +1 | Higher | -10 | Lower |
| PLOC | 48 | **36 ✅** | 50 | +12 | Higher | -2 | Lower | -14 | Lower |
| LLOC | 34 | **29 ✅** | 42 | +5 | Higher | -8 | Lower | -13 | Lower |
| Comments (CLOC) | 7 | **9 ✅** | 1 | -2 | Lower | +6 | Higher | +8 | Higher |
| Blank Lines | **10 ✅** | 11 | 13 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00479

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.1819 | 64.9295 | **77.0343 ✅** | +9.2524 | Higher | -2.8524 | Lower | -12.1048 | Lower |
| MI (SEI) | 60.5805 | 58.8405 | **66.7258 ✅** | +1.7400 | Higher | -6.1453 | Lower | -7.8852 | Lower |
| Cyclomatic (sum) | 7 | 11 | **6 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Cognitive (sum) | 8 | **5 ✅** | 10 | +3 | Higher | -2 | Lower | -5 | Lower |
| Halstead Volume | **988.1571 ✅** | 1780.0667 | 1004.2264 | -791.9096 | Lower | -16.0693 | Lower | +775.8403 | Higher |
| Halstead Effort | **27066.9125 ✅** | 31896.0866 | 27189.4304 | -4829.1741 | Lower | -122.5179 | Lower | +4706.6563 | Higher |
| SLOC | 39 | 54 | **33 ✅** | -15 | Lower | +6 | Higher | +21 | Higher |
| PLOC | 26 | 32 | **21 ✅** | -6 | Lower | +5 | Higher | +11 | Higher |
| LLOC | 15 | **11 ✅** | 13 | +4 | Higher | +2 | Higher | -2 | Lower |
| Comments (CLOC) | 6 | **19 ✅** | 6 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | 7 | 8 | **6 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p00492

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.3844 | **63.6446 ✅** | 58.2996 | -9.2601 | Lower | -3.9152 | Lower | +5.3449 | Higher |
| MI (SEI) | 31.5663 | **49.7853 ✅** | 19.7891 | -18.2190 | Lower | +11.7771 | Higher | +29.9961 | Higher |
| Cyclomatic (sum) | 19 | **17 ✅** | 18 | +2 | Higher | +1 | Higher | -1 | Lower |
| Cognitive (sum) | 28 | **24 ✅** | 26 | +4 | Higher | +2 | Higher | -2 | Lower |
| Halstead Volume | 2583.9058 | **1650.7247 ✅** | 2623.2670 | +933.1812 | Higher | -39.3612 | Lower | -972.5424 | Lower |
| Halstead Effort | 181013.0798 | **135235.6206 ✅** | 161401.8225 | +45777.4592 | Higher | +19611.2572 | Higher | -26166.2020 | Lower |
| SLOC | 82 | **55 ✅** | 65 | +27 | Higher | +17 | Higher | -10 | Lower |
| PLOC | 61 | **41 ✅** | 56 | +20 | Higher | +5 | Higher | -15 | Lower |
| LLOC | 51 | **20 ✅** | 51 | +31 | Higher | 0 | Same | -31 | Lower |
| Comments (CLOC) | 11 | 11 | 1 | 0 | Same | +10 | Higher | +10 | Higher |
| Blank Lines | 12 | **7 ✅** | 9 | +5 | Higher | +3 | Higher | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p00495

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.1997 | 61.8558 | 64.1997 | +2.3439 | Higher | 0 | Same | -2.3439 | Lower |
| MI (SEI) | 18.2434 | **47.5562 ✅** | 18.2434 | -29.3128 | Lower | 0 | Same | +29.3128 | Higher |
| Cyclomatic (sum) | 13 | 16 | 13 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 15 | 17 | 15 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 1674.0394 | 2062.6285 | 1674.0394 | -388.5891 | Lower | 0 | Same | +388.5891 | Higher |
| Halstead Effort | 73825.1390 | **52742.6251 ✅** | 73825.1390 | +21082.5139 | Higher | 0 | Same | -21082.5139 | Lower |
| SLOC | 56 | 58 | 56 | -2 | Lower | 0 | Same | +2 | Higher |
| PLOC | 47 | **42 ✅** | 47 | +5 | Higher | 0 | Same | -5 | Lower |
| LLOC | 35 | **27 ✅** | 35 | +8 | Higher | 0 | Same | -8 | Lower |
| Comments (CLOC) | 0 | **12 ✅** | 0 | -12 | Lower | 0 | Same | +12 | Higher |
| Blank Lines | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

## p00507

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.2381 | 63.6495 | **72.3125 ✅** | +3.5885 | Higher | -5.0745 | Lower | -8.6630 | Lower |
| MI (SEI) | **55.2231 ✅** | 54.3335 | 41.7676 | +0.8895 | Higher | +13.4555 | Higher | +12.5659 | Higher |
| Cyclomatic (sum) | **8 ✅** | 10 | 10 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | 10 | **8 ✅** | 20 | +2 | Higher | -10 | Lower | -12 | Lower |
| Halstead Volume | 1231.5029 | 1904.8783 | **1145.7028 ✅** | -673.3754 | Lower | +85.8001 | Higher | +759.1755 | Higher |
| Halstead Effort | **27477.9084 ✅** | 32645.6725 | 42963.8552 | -5167.7641 | Lower | -15485.9468 | Lower | -10318.1827 | Lower |
| SLOC | 55 | 58 | **40 ✅** | -3 | Lower | +15 | Higher | +18 | Higher |
| PLOC | 36 | 38 | **34 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| LLOC | 23 | **14 ✅** | 26 | +9 | Higher | -3 | Lower | -12 | Lower |
| Comments (CLOC) | 12 | **17 ✅** | 1 | -5 | Lower | +11 | Higher | +16 | Higher |
| Blank Lines | 10 | 10 | **5 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| Functions | **1 ✅** | 4 | 2 | -3 | Lower | -1 | Lower | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p00514

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.7048 | **67.5081 ✅** | 52.3221 | -15.8034 | Lower | -0.6173 | Lower | +15.1861 | Higher |
| MI (SEI) | 1.0316 | **53.6295 ✅** | 10.0960 | -52.5979 | Lower | -9.0645 | Lower | +43.5335 | Higher |
| Cyclomatic (sum) | 21 | **10 ✅** | 21 | +11 | Higher | 0 | Same | -11 | Lower |
| Cognitive (sum) | 19 | **6 ✅** | 18 | +13 | Higher | +1 | Higher | -12 | Lower |
| Halstead Volume | 2675.1400 | **1133.1465 ✅** | 2724.4019 | +1541.9935 | Higher | -49.2619 | Lower | -1591.2554 | Lower |
| Halstead Effort | 141098.0817 | **39953.9064 ✅** | 177803.0719 | +101144.1753 | Higher | -36704.9902 | Lower | -137849.1655 | Lower |
| SLOC | 93 | **54 ✅** | 89 | +39 | Higher | +4 | Higher | -35 | Lower |
| PLOC | 83 | **36 ✅** | 80 | +47 | Higher | +3 | Higher | -44 | Lower |
| LLOC | 55 | **18 ✅** | 55 | +37 | Higher | 0 | Same | -37 | Lower |
| Comments (CLOC) | 0 | **10 ✅** | 1 | -10 | Lower | -1 | Lower | +9 | Higher |
| Blank Lines | 10 | **8 ✅** | 9 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p00518

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.2940 | 61.4796 | **62.3942 ✅** | -3.1855 | Lower | -4.1002 | Lower | -0.9147 | Lower |
| MI (SEI) | 33.5045 | **55.7365 ✅** | 29.6848 | -22.2319 | Lower | +3.8197 | Higher | +26.0516 | Higher |
| Cyclomatic (sum) | 17 | **12 ✅** | 15 | +5 | Higher | +2 | Higher | -3 | Lower |
| Cognitive (sum) | 27 | **3 ✅** | 27 | +24 | Higher | 0 | Same | -24 | Lower |
| Halstead Volume | 2084.7643 | 2381.2721 | **1661.2208 ✅** | -296.5078 | Lower | +423.5435 | Higher | +720.0513 | Higher |
| Halstead Effort | 86881.3390 | **49097.5002 ✅** | 73722.9642 | +37783.8388 | Higher | +13158.3748 | Higher | -24625.4640 | Lower |
| SLOC | 71 | **60 ✅** | 61 | +11 | Higher | +10 | Higher | -1 | Lower |
| PLOC | 54 | **39 ✅** | 49 | +15 | Higher | +5 | Higher | -10 | Lower |
| LLOC | 45 | **18 ✅** | 41 | +27 | Higher | +4 | Higher | -23 | Lower |
| Comments (CLOC) | 7 | **24 ✅** | 2 | -17 | Lower | +5 | Higher | +22 | Higher |
| Blank Lines | 14 | **7 ✅** | 12 | +7 | Higher | +2 | Higher | -5 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00519

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **48.0124 ✅** | 43.9093 | 35.1842 | +4.1031 | Higher | +12.8282 | Higher | +8.7251 | Higher |
| MI (SEI) | 5.7129 | **10.3835 ✅** | -15.8070 | -4.6706 | Lower | +21.5199 | Higher | +26.1905 | Higher |
| Cyclomatic (sum) | **17 ✅** | 25 | 32 | -8 | Lower | -15 | Lower | -7 | Lower |
| Cognitive (sum) | 18 | **13 ✅** | 29 | +5 | Higher | -11 | Lower | -16 | Lower |
| Halstead Volume | **3960.7549 ✅** | 3994.5923 | 5701.8799 | -33.8374 | Lower | -1741.1250 | Lower | -1707.2876 | Lower |
| Halstead Effort | 167637.2139 | **119774.8620 ✅** | 445281.1823 | +47862.3519 | Higher | -277643.9684 | Lower | -325506.3203 | Lower |
| SLOC | **109 ✅** | 125 | 173 | -16 | Lower | -64 | Lower | -48 | Lower |
| PLOC | 93 | **92 ✅** | 149 | +1 | Higher | -56 | Lower | -57 | Lower |
| LLOC | 52 | **25 ✅** | 94 | +27 | Higher | -42 | Lower | -69 | Lower |
| Comments (CLOC) | 2 | **9 ✅** | 1 | -7 | Lower | +1 | Higher | +8 | Higher |
| Blank Lines | **16 ✅** | 24 | 23 | -8 | Lower | -7 | Lower | +1 | Higher |
| Functions | 5 | 5 | 8 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |
| Exits | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |

## p00528

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.3287 | **95.7796 ✅** | 24.4516 | -56.4509 | Lower | +14.8771 | Higher | +71.3280 | Higher |
| MI (SEI) | -9.4022 | **83.3243 ✅** | -27.3304 | -92.7265 | Lower | +17.9282 | Higher | +110.6547 | Higher |
| Cyclomatic (sum) | 30 | **27 ✅** | 45 | +3 | Higher | -15 | Lower | -18 | Lower |
| Cognitive (sum) | 26 | **23 ✅** | 47 | +3 | Higher | -21 | Lower | -24 | Lower |
| Halstead Volume | 5309.2170 | **3855.3577 ✅** | 8440.3167 | +1453.8592 | Higher | -3131.0997 | Lower | -4584.9589 | Lower |
| Halstead Effort | 328068.7686 | **106340.8237 ✅** | 953293.3003 | +221727.9450 | Higher | -625224.5317 | Lower | -846952.4766 | Lower |
| SLOC | 141 | **5 ✅** | 246 | +136 | Higher | -105 | Lower | -241 | Lower |
| PLOC | 121 | **69 ✅** | 211 | +52 | Higher | -90 | Lower | -142 | Lower |
| LLOC | 80 | **35 ✅** | 174 | +45 | Higher | -94 | Lower | -139 | Lower |
| Comments (CLOC) | 1 | **16 ✅** | 3 | -15 | Lower | -2 | Lower | +13 | Higher |
| Blank Lines | 20 | **-70 ✅** | 32 | +90 | Higher | -12 | Lower | -102 | Lower |
| Functions | 4 | 4 | 11 | 0 | Same | -7 | Lower | -7 | Lower |
| Closures | 4 | **0 ✅** | 2 | +4 | Higher | +2 | Higher | -2 | Lower |
| Exits | 4 | 4 | 8 | 0 | Same | -4 | Lower | -4 | Lower |

## p00534

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.6067 | **70.4027 ✅** | 68.2357 | -4.7961 | Lower | -2.6290 | Lower | +2.1671 | Higher |
| MI (SEI) | 49.7631 | **64.7089 ✅** | 50.1970 | -14.9458 | Lower | -0.4339 | Lower | +14.5119 | Higher |
| Cyclomatic (sum) | 10 | **8 ✅** | 9 | +2 | Higher | +1 | Higher | -1 | Lower |
| Cognitive (sum) | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 1731.3384 | **1252.0803 ✅** | 1696.9826 | +479.2581 | Higher | +34.3558 | Higher | -444.9023 | Lower |
| Halstead Effort | 72716.2118 | **48460.1448 ✅** | 72758.1293 | +24256.0670 | Higher | -41.9175 | Lower | -24297.9845 | Lower |
| SLOC | 53 | **45 ✅** | 46 | +8 | Higher | +7 | Higher | -1 | Lower |
| PLOC | 38 | **28 ✅** | 32 | +10 | Higher | +6 | Higher | -4 | Lower |
| LLOC | 30 | **20 ✅** | 28 | +10 | Higher | +2 | Higher | -8 | Lower |
| Comments (CLOC) | 9 | **14 ✅** | 6 | -5 | Lower | +3 | Higher | +8 | Higher |
| Blank Lines | 8 | **6 ✅** | 8 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00543

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.6788 | 70.8227 | **74.8361 ✅** | +1.8562 | Higher | -2.1573 | Lower | -4.0135 | Lower |
| MI (SEI) | 58.0982 | **64.1439 ✅** | 55.0896 | -6.0457 | Lower | +3.0087 | Higher | +9.0543 | Higher |
| Cyclomatic (sum) | 10 | 10 | **9 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 14 | **5 ✅** | 9 | +9 | Higher | +5 | Higher | -4 | Lower |
| Halstead Volume | **988.7176 ✅** | 1651.0431 | 1117.3273 | -662.3255 | Lower | -128.6097 | Lower | +533.7158 | Higher |
| Halstead Effort | **31818.7297 ✅** | 36883.1225 | 37244.2420 | -5064.3928 | Lower | -5425.5122 | Lower | -361.1194 | Lower |
| SLOC | 41 | 39 | **35 ✅** | +2 | Higher | +6 | Higher | +4 | Higher |
| PLOC | 29 | **26 ✅** | 28 | +3 | Higher | +1 | Higher | -2 | Lower |
| LLOC | 20 | **12 ✅** | 22 | +8 | Higher | -2 | Lower | -10 | Lower |
| Comments (CLOC) | 6 | **11 ✅** | 3 | -5 | Lower | +3 | Higher | +8 | Higher |
| Blank Lines | 7 | 6 | **5 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p00554

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.1132 | **67.0493 ✅** | 66.1460 | -2.9362 | Lower | -2.0329 | Lower | +0.9033 | Higher |
| MI (SEI) | 44.9158 | **51.7550 ✅** | 39.1284 | -6.8391 | Lower | +5.7875 | Higher | +12.6266 | Higher |
| Cyclomatic (sum) | 11 | **5 ✅** | 12 | +6 | Higher | -1 | Lower | -7 | Lower |
| Cognitive (sum) | 17 | **7 ✅** | 17 | +10 | Higher | 0 | Same | -10 | Lower |
| Halstead Volume | 1580.5444 | **1304.6578 ✅** | 1515.9717 | +275.8866 | Higher | +64.5727 | Higher | -211.3138 | Lower |
| Halstead Effort | 70523.8906 | **29049.0220 ✅** | 63029.4370 | +41474.8686 | Higher | +7494.4536 | Higher | -33980.4150 | Lower |
| SLOC | 59 | 57 | **52 ✅** | +2 | Higher | +7 | Higher | +5 | Higher |
| PLOC | 41 | **35 ✅** | 40 | +6 | Higher | +1 | Higher | -5 | Lower |
| LLOC | 37 | **26 ✅** | 36 | +11 | Higher | +1 | Higher | -10 | Lower |
| Comments (CLOC) | 8 | **10 ✅** | 3 | -2 | Lower | +5 | Higher | +7 | Higher |
| Blank Lines | 10 | 12 | **9 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00571

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.3570 | **67.8961 ✅** | 59.3266 | -2.5392 | Lower | +6.0304 | Higher | +8.5695 | Higher |
| MI (SEI) | 45.5882 | **56.2180 ✅** | 23.9657 | -10.6299 | Lower | +21.6224 | Higher | +32.2523 | Higher |
| Cyclomatic (sum) | 15 | **6 ✅** | 12 | +9 | Higher | +3 | Higher | -6 | Lower |
| Cognitive (sum) | 15 | **1 ✅** | 9 | +14 | Higher | +6 | Higher | -8 | Lower |
| Halstead Volume | 3005.5734 | 2215.1194 | **2132.3875 ✅** | +790.4540 | Higher | +873.1859 | Higher | +82.7319 | Higher |
| Halstead Effort | 74423.7229 | **36900.9074 ✅** | 103808.5008 | +37522.8155 | Higher | -29384.7779 | Lower | -66907.5935 | Lower |
| SLOC | **42 ✅** | 45 | 71 | -3 | Lower | -29 | Lower | -26 | Lower |
| PLOC | 42 | **27 ✅** | 58 | +15 | Higher | -16 | Lower | -31 | Lower |
| LLOC | 26 | **18 ✅** | 40 | +8 | Higher | -14 | Lower | -22 | Lower |
| Comments (CLOC) | 5 | **10 ✅** | 2 | -5 | Lower | +3 | Higher | +8 | Higher |
| Blank Lines | **-2 ✅** | 10 | 11 | -12 | Lower | -13 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 4 | +1 | Higher | -2 | Lower | -3 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 4 | 0 | Same | -4 | Lower | -4 | Lower |

## p00594

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.9687 | **68.5580 ✅** | 65.3558 | -1.5892 | Lower | +1.6129 | Higher | +3.2022 | Higher |
| MI (SEI) | 47.0345 | **52.1635 ✅** | 45.9485 | -5.1290 | Lower | +1.0860 | Higher | +6.2150 | Higher |
| Cyclomatic (sum) | **12 ✅** | 13 | 13 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 24 | **15 ✅** | 26 | +9 | Higher | -2 | Lower | -11 | Lower |
| Halstead Volume | 1219.5714 | **1097.5901 ✅** | 1340.3334 | +121.9814 | Higher | -120.7619 | Lower | -242.7433 | Lower |
| Halstead Effort | 35062.6791 | **34233.4561 ✅** | 63799.8692 | +829.2230 | Higher | -28737.1900 | Lower | -29566.4131 | Lower |
| SLOC | 53 | **49 ✅** | 56 | +4 | Higher | -3 | Lower | -7 | Lower |
| PLOC | 42 | **36 ✅** | 47 | +6 | Higher | -5 | Lower | -11 | Lower |
| LLOC | 27 | **24 ✅** | 35 | +3 | Higher | -8 | Lower | -11 | Lower |
| Comments (CLOC) | 6 | 7 | 7 | -1 | Lower | -1 | Lower | 0 | Same |
| Blank Lines | **7 ✅** | 9 | 8 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00629

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.0733 | **58.8431 ✅** | 57.2407 | -1.7698 | Lower | -0.1673 | Lower | +1.6025 | Higher |
| MI (SEI) | 17.1511 | **31.8013 ✅** | 18.4951 | -14.6502 | Lower | -1.3441 | Lower | +13.3062 | Higher |
| Cyclomatic (sum) | 17 | **16 ✅** | 21 | +1 | Higher | -4 | Lower | -5 | Lower |
| Cognitive (sum) | 21 | **15 ✅** | 19 | +6 | Higher | +2 | Higher | -4 | Lower |
| Halstead Volume | **2047.6051 ✅** | 2709.2285 | 2685.3425 | -661.6234 | Lower | -637.7375 | Lower | +23.8859 | Higher |
| Halstead Effort | 76145.3130 | **58323.6681 ✅** | 122112.4185 | +17821.6449 | Higher | -45967.1054 | Lower | -63788.7503 | Lower |
| SLOC | 77 | **64 ✅** | 66 | +13 | Higher | +11 | Higher | -2 | Lower |
| PLOC | 64 | **51 ✅** | 57 | +13 | Higher | +7 | Higher | -6 | Lower |
| LLOC | 42 | **15 ✅** | 41 | +27 | Higher | +1 | Higher | -26 | Lower |
| Comments (CLOC) | 1 | **5 ✅** | 1 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 12 | 9 | **8 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 5 | **0 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Exits | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |

## p00641

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.1762 | **50.9230 ✅** | 47.4195 | -6.7468 | Lower | -3.2432 | Lower | +3.5035 | Higher |
| MI (SEI) | 3.2212 | **22.3534 ✅** | 9.6746 | -19.1322 | Lower | -6.4534 | Lower | +12.6788 | Higher |
| Cyclomatic (sum) | 26 | **15 ✅** | 23 | +11 | Higher | +3 | Higher | -8 | Lower |
| Cognitive (sum) | 16 | **11 ✅** | 15 | +5 | Higher | +1 | Higher | -4 | Lower |
| Halstead Volume | 5111.5024 | 3921.2313 | **3404.4035 ✅** | +1190.2711 | Higher | +1707.0989 | Higher | +516.8278 | Higher |
| Halstead Effort | 187132.9705 | **109678.0047 ✅** | 282267.6058 | +77454.9659 | Higher | -95134.6353 | Lower | -172589.6011 | Lower |
| SLOC | 112 | **94 ✅** | 109 | +18 | Higher | +3 | Higher | -15 | Lower |
| PLOC | 95 | **75 ✅** | 92 | +20 | Higher | +3 | Higher | -17 | Lower |
| LLOC | 58 | **38 ✅** | 67 | +20 | Higher | -9 | Lower | -29 | Lower |
| Comments (CLOC) | 3 | **9 ✅** | 4 | -6 | Lower | -1 | Lower | +5 | Higher |
| Blank Lines | 15 | 15 | 15 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |

## p00649

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.5251 | **60.0671 ✅** | 46.1897 | -10.5420 | Lower | +3.3354 | Higher | +13.8774 | Higher |
| MI (SEI) | -1.8076 | **34.5486 ✅** | 7.7284 | -36.3562 | Lower | -9.5360 | Lower | +26.8202 | Higher |
| Cyclomatic (sum) | 24 | **9 ✅** | 25 | +15 | Higher | -1 | Lower | -16 | Lower |
| Cognitive (sum) | 23 | **2 ✅** | 30 | +21 | Higher | -7 | Lower | -28 | Lower |
| Halstead Volume | **2754.9445 ✅** | 2780.4204 | 3340.6859 | -25.4758 | Lower | -585.7414 | Lower | -560.2655 | Lower |
| Halstead Effort | 107259.1737 | **50542.3874 ✅** | 222777.1360 | +56716.7863 | Higher | -115517.9623 | Lower | -172234.7487 | Lower |
| SLOC | 101 | **65 ✅** | 115 | +36 | Higher | -14 | Lower | -50 | Lower |
| PLOC | 89 | **50 ✅** | 94 | +39 | Higher | -5 | Lower | -44 | Lower |
| LLOC | 50 | **16 ✅** | 70 | +34 | Higher | -20 | Lower | -54 | Lower |
| Comments (CLOC) | 0 | **6 ✅** | 4 | -6 | Lower | -4 | Lower | +2 | Higher |
| Blank Lines | 12 | **10 ✅** | 19 | +2 | Higher | -7 | Lower | -9 | Lower |
| Functions | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p00653

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.2787 | **59.3379 ✅** | 43.7499 | -17.0592 | Lower | -1.4711 | Lower | +15.5880 | Higher |
| MI (SEI) | 3.3907 | **35.6339 ✅** | 0.5001 | -32.2432 | Lower | +2.8906 | Higher | +35.1338 | Higher |
| Cyclomatic (sum) | 28 | **18 ✅** | 28 | +10 | Higher | 0 | Same | -10 | Lower |
| Cognitive (sum) | 30 | **10 ✅** | 30 | +20 | Higher | 0 | Same | -20 | Lower |
| Halstead Volume | 4786.5868 | **3064.0177 ✅** | 4939.8179 | +1722.5691 | Higher | -153.2312 | Lower | -1875.8002 | Lower |
| Halstead Effort | 414638.0774 | **80430.4641 ✅** | 440803.1388 | +334207.6133 | Higher | -26165.0614 | Lower | -360372.6747 | Lower |
| SLOC | 125 | **58 ✅** | 113 | +67 | Higher | +12 | Higher | -55 | Lower |
| PLOC | 102 | **46 ✅** | 91 | +56 | Higher | +11 | Higher | -45 | Lower |
| LLOC | 82 | **21 ✅** | 80 | +61 | Higher | +2 | Higher | -59 | Lower |
| Comments (CLOC) | 5 | **6 ✅** | 2 | -1 | Lower | +3 | Higher | +4 | Higher |
| Blank Lines | 21 | **7 ✅** | 20 | +14 | Higher | +1 | Higher | -13 | Lower |
| Functions | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 7 | **3 ✅** | 7 | +4 | Higher | 0 | Same | -4 | Lower |

## p00655

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 29.0438 | **42.4114 ✅** | 29.9915 | -13.3676 | Lower | -0.9477 | Lower | +12.4198 | Higher |
| MI (SEI) | -18.7614 | **1.3261 ✅** | -16.6182 | -20.0876 | Lower | -2.1432 | Lower | +17.9444 | Higher |
| Cyclomatic (sum) | 42 | **35 ✅** | 45 | +7 | Higher | -3 | Lower | -10 | Lower |
| Cognitive (sum) | 39 | **15 ✅** | 41 | +24 | Higher | -2 | Lower | -26 | Lower |
| Halstead Volume | 7140.8714 | **4207 ✅** | 6834.3583 | +2933.8714 | Higher | +306.5131 | Higher | -2627.3583 | Lower |
| Halstead Effort | 954893.1883 | **112132.7308 ✅** | 942073.5773 | +842760.4575 | Higher | +12819.6109 | Higher | -829940.8466 | Lower |
| SLOC | 204 | **117 ✅** | 187 | +87 | Higher | +17 | Higher | -70 | Lower |
| PLOC | 185 | **101 ✅** | 170 | +84 | Higher | +15 | Higher | -69 | Lower |
| LLOC | 127 | **34 ✅** | 117 | +93 | Higher | +10 | Higher | -83 | Lower |
| Comments (CLOC) | 4 | 3 | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Blank Lines | 17 | **13 ✅** | 15 | +4 | Higher | +2 | Higher | -2 | Lower |
| Functions | 9 | **8 ✅** | 10 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 8 | **3 ✅** | 9 | +5 | Higher | -1 | Lower | -6 | Lower |

## p00658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | -3.0333 | **48.7713 ✅** | -7.9886 | -51.8046 | Lower | +4.9553 | Higher | +56.7599 | Higher |
| MI (SEI) | -60.9732 | **20.9500 ✅** | -64.7012 | -81.9233 | Lower | +3.7280 | Higher | +85.6513 | Higher |
| Cyclomatic (sum) | 130 | **19 ✅** | 144 | +111 | Higher | -14 | Lower | -125 | Lower |
| Cognitive (sum) | 176 | **8 ✅** | 189 | +168 | Higher | -13 | Lower | -181 | Lower |
| Halstead Volume | 13293.6186 | **3852.8203 ✅** | 13426.8231 | +9440.7983 | Higher | -133.2044 | Lower | -9574.0028 | Lower |
| Halstead Effort | 1919878.3921 | **130695.4394 ✅** | 1910189.3597 | +1789182.9527 | Higher | +9689.0324 | Higher | -1779493.9203 | Lower |
| SLOC | 347 | **102 ✅** | 385 | +245 | Higher | -38 | Lower | -283 | Lower |
| PLOC | 326 | **85 ✅** | 366 | +241 | Higher | -40 | Lower | -281 | Lower |
| LLOC | 227 | **34 ✅** | 262 | +193 | Higher | -35 | Lower | -228 | Lower |
| Comments (CLOC) | 2 | **11 ✅** | 4 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | 20 | **13 ✅** | 15 | +7 | Higher | +5 | Higher | -2 | Lower |
| Functions | 12 | **5 ✅** | 10 | +7 | Higher | +2 | Higher | -5 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 27 | **6 ✅** | 26 | +21 | Higher | +1 | Higher | -20 | Lower |

## p00680

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 24.6393 | **46.8566 ✅** | 23.8674 | -22.2174 | Lower | +0.7718 | Higher | +22.9892 | Higher |
| MI (SEI) | -26.2876 | **23.6520 ✅** | -29.7932 | -49.9396 | Lower | +3.5056 | Higher | +53.4452 | Higher |
| Cyclomatic (sum) | 49 | **24 ✅** | 45 | +25 | Higher | +4 | Higher | -21 | Lower |
| Cognitive (sum) | 61 | **24 ✅** | 74 | +37 | Higher | -13 | Lower | -50 | Lower |
| Halstead Volume | 8883.3816 | **4201.1739 ✅** | 8870.0185 | +4682.2077 | Higher | +13.3631 | Higher | -4668.8446 | Lower |
| Halstead Effort | 868296.3827 | **198505.4668 ✅** | 914245.4788 | +669790.9160 | Higher | -45949.0961 | Lower | -715740.0120 | Lower |
| SLOC | 226 | **104 ✅** | 251 | +122 | Higher | -25 | Lower | -147 | Lower |
| PLOC | 195 | **86 ✅** | 220 | +109 | Higher | -25 | Lower | -134 | Lower |
| LLOC | 133 | **47 ✅** | 155 | +86 | Higher | -22 | Lower | -108 | Lower |
| Comments (CLOC) | 3 | **17 ✅** | 2 | -14 | Lower | +1 | Higher | +15 | Higher |
| Blank Lines | 29 | **12 ✅** | 29 | +17 | Higher | 0 | Same | -17 | Lower |
| Functions | 8 | **6 ✅** | 7 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 2 | 3 | **0 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Exits | 8 | **6 ✅** | 8 | +2 | Higher | 0 | Same | -2 | Lower |

## p00690

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.9855 | 48.9568 | **50.3407 ✅** | +0.0287 | Higher | -1.3552 | Lower | -1.3839 | Lower |
| MI (SEI) | -2.4842 | **11.5867 ✅** | 10.9785 | -14.0708 | Lower | -13.4627 | Lower | +0.6082 | Higher |
| Cyclomatic (sum) | 25 | **16 ✅** | 25 | +9 | Higher | 0 | Same | -9 | Lower |
| Cognitive (sum) | 38 | **10 ✅** | 38 | +28 | Higher | 0 | Same | -28 | Lower |
| Halstead Volume | 3424.9814 | 4118.2543 | **3341.2995 ✅** | -693.2729 | Lower | +83.6819 | Higher | +776.9549 | Higher |
| Halstead Effort | 221947.8076 | **105957.7981 ✅** | 197393.6922 | +115990.0095 | Higher | +24554.1154 | Higher | -91435.8941 | Lower |
| SLOC | 96 | 103 | **89 ✅** | -7 | Lower | +7 | Higher | +14 | Higher |
| PLOC | 87 | 84 | **76 ✅** | +3 | Higher | +11 | Higher | +8 | Higher |
| LLOC | 63 | **36 ✅** | 58 | +27 | Higher | +5 | Higher | -22 | Lower |
| Comments (CLOC) | 0 | **4 ✅** | 2 | -4 | Lower | -2 | Lower | +2 | Higher |
| Blank Lines | **9 ✅** | 15 | 11 | -6 | Lower | -2 | Lower | +4 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p00692

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.3499 | **72.3192 ✅** | 51.2923 | -14.9693 | Lower | +6.0576 | Higher | +21.0269 | Higher |
| MI (SEI) | 30.6169 | **61.7857 ✅** | 23.3703 | -31.1689 | Lower | +7.2465 | Higher | +38.4154 | Higher |
| Cyclomatic (sum) | 24 | **9 ✅** | 29 | +15 | Higher | -5 | Lower | -20 | Lower |
| Cognitive (sum) | 20 | **12 ✅** | 28 | +8 | Higher | -8 | Lower | -16 | Lower |
| Halstead Volume | 2661.8866 | **888.7342 ✅** | 3249.7350 | +1773.1524 | Higher | -587.8484 | Lower | -2361.0008 | Lower |
| Halstead Effort | 151727.5347 | **18770.0666 ✅** | 234139.4445 | +132957.4680 | Higher | -82411.9098 | Lower | -215369.3778 | Lower |
| SLOC | 63 | **44 ✅** | 80 | +19 | Higher | -17 | Lower | -36 | Lower |
| PLOC | 50 | **30 ✅** | 64 | +20 | Higher | -14 | Lower | -34 | Lower |
| LLOC | 37 | **17 ✅** | 47 | +20 | Higher | -10 | Lower | -30 | Lower |
| Comments (CLOC) | 5 | **9 ✅** | 7 | -4 | Lower | -2 | Lower | +2 | Higher |
| Blank Lines | 8 | 8 | 9 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00723

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.4434 | **62.0503 ✅** | 50.0945 | -8.6069 | Lower | +3.3489 | Higher | +11.9558 | Higher |
| MI (SEI) | 3.0308 | **39.2288 ✅** | 6.3602 | -36.1980 | Lower | -3.3294 | Lower | +32.8686 | Higher |
| Cyclomatic (sum) | 16 | **13 ✅** | 20 | +3 | Higher | -4 | Lower | -7 | Lower |
| Cognitive (sum) | 28 | **7 ✅** | 26 | +21 | Higher | +2 | Higher | -19 | Lower |
| Halstead Volume | 3161.4264 | **2395.1675 ✅** | 3136.5589 | +766.2589 | Higher | +24.8675 | Higher | -741.3914 | Lower |
| Halstead Effort | 154006.6283 | **45650.4698 ✅** | 170603.3729 | +108356.1586 | Higher | -16596.7446 | Lower | -124952.9032 | Lower |
| SLOC | 85 | **57 ✅** | 99 | +28 | Higher | -14 | Lower | -42 | Lower |
| PLOC | 70 | **40 ✅** | 86 | +30 | Higher | -16 | Lower | -46 | Lower |
| LLOC | 48 | **20 ✅** | 48 | +28 | Higher | 0 | Same | -28 | Lower |
| Comments (CLOC) | 0 | **6 ✅** | 1 | -6 | Lower | -1 | Lower | +5 | Higher |
| Blank Lines | 15 | **12 ✅** | 13 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 4 | **2 ✅** | 6 | +2 | Higher | -2 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |

## p00740

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.4360 | **74.2029 ✅** | 60.7111 | -18.7669 | Lower | -5.2751 | Lower | +13.4918 | Higher |
| MI (SEI) | 19.2692 | **63.4226 ✅** | 22.7523 | -44.1534 | Lower | -3.4831 | Lower | +40.6703 | Higher |
| Cyclomatic (sum) | 21 | **12 ✅** | 15 | +9 | Higher | +6 | Higher | -3 | Lower |
| Cognitive (sum) | 27 | **10 ✅** | 16 | +17 | Higher | +11 | Higher | -6 | Lower |
| Halstead Volume | 3026.3307 | **855.4067 ✅** | 1636.8835 | +2170.9240 | Higher | +1389.4472 | Higher | -781.4768 | Lower |
| Halstead Effort | 86126.3959 | **24546.4526 ✅** | 53994.4212 | +61579.9432 | Higher | +32131.9747 | Higher | -29447.9685 | Lower |
| SLOC | 71 | **38 ✅** | 68 | +33 | Higher | +3 | Higher | -30 | Lower |
| PLOC | 56 | **28 ✅** | 58 | +28 | Higher | -2 | Lower | -30 | Lower |
| LLOC | 34 | **18 ✅** | 37 | +16 | Higher | -3 | Lower | -19 | Lower |
| Comments (CLOC) | 2 | **7 ✅** | 1 | -5 | Lower | +1 | Higher | +6 | Higher |
| Blank Lines | 15 | **7 ✅** | 10 | +8 | Higher | +5 | Higher | -3 | Lower |
| Functions | 1 | 1 | 5 | 0 | Same | -4 | Lower | -4 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p00749

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 22.9613 | **57.2940 ✅** | 25.1417 | -34.3327 | Lower | -2.1804 | Lower | +32.1523 | Higher |
| MI (SEI) | -29.8977 | **38.1705 ✅** | -27.0770 | -68.0683 | Lower | -2.8207 | Lower | +65.2476 | Higher |
| Cyclomatic (sum) | 55 | **20 ✅** | 50 | +35 | Higher | +5 | Higher | -30 | Lower |
| Cognitive (sum) | 74 | **21 ✅** | 81 | +53 | Higher | -7 | Lower | -60 | Lower |
| Halstead Volume | 8006.9104 | **2531.3939 ✅** | 7716.2216 | +5475.5165 | Higher | +290.6888 | Higher | -5184.8277 | Lower |
| Halstead Effort | 750560.8193 | **132476.2800 ✅** | 679896.9350 | +618084.5393 | Higher | +70663.8844 | Higher | -547420.6549 | Lower |
| SLOC | 238 | **68 ✅** | 226 | +170 | Higher | +12 | Higher | -158 | Lower |
| PLOC | 211 | **50 ✅** | 199 | +161 | Higher | +12 | Higher | -149 | Lower |
| LLOC | 135 | **38 ✅** | 135 | +97 | Higher | 0 | Same | -97 | Lower |
| Comments (CLOC) | 2 | **11 ✅** | 2 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 25 | **13 ✅** | 25 | +12 | Higher | 0 | Same | -12 | Lower |
| Functions | 8 | **3 ✅** | 7 | +5 | Higher | +1 | Higher | -4 | Lower |
| Closures | 3 | 1 | **0 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Exits | 13 | **3 ✅** | 13 | +10 | Higher | 0 | Same | -10 | Lower |

## p00762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.0089 | **70.9381 ✅** | 38.6169 | -31.9292 | Lower | +0.3921 | Higher | +32.3212 | Higher |
| MI (SEI) | -3.6444 | **58.5369 ✅** | 4.1344 | -62.1813 | Lower | -7.7788 | Lower | +54.4025 | Higher |
| Cyclomatic (sum) | 30 | **9 ✅** | 31 | +21 | Higher | -1 | Lower | -22 | Lower |
| Cognitive (sum) | 19 | **6 ✅** | 19 | +13 | Higher | 0 | Same | -13 | Lower |
| Halstead Volume | 5174.6933 | **1245.1452 ✅** | 5824.6249 | +3929.5481 | Higher | -649.9316 | Lower | -4579.4797 | Lower |
| Halstead Effort | 462309.3042 | **25970.1711 ✅** | 602538.8577 | +436339.1331 | Higher | -140229.5535 | Lower | -576568.6866 | Lower |
| SLOC | 145 | **43 ✅** | 141 | +102 | Higher | +4 | Higher | -98 | Lower |
| PLOC | 116 | **28 ✅** | 113 | +88 | Higher | +3 | Higher | -85 | Lower |
| LLOC | 85 | **18 ✅** | 96 | +67 | Higher | -11 | Lower | -78 | Lower |
| Comments (CLOC) | 4 | 8 | **11 ✅** | -4 | Lower | -7 | Lower | -3 | Lower |
| Blank Lines | 25 | **9 ✅** | 24 | +16 | Higher | +1 | Higher | -15 | Lower |
| Functions | 8 | **1 ✅** | 8 | +7 | Higher | 0 | Same | -7 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 5 | **0 ✅** | 4 | +5 | Higher | +1 | Higher | -4 | Lower |

## p00792

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.2911 | **59.0009 ✅** | 55.8193 | -3.7098 | Lower | -0.5282 | Lower | +3.1816 | Higher |
| MI (SEI) | 23.4006 | **45.9311 ✅** | 26.4852 | -22.5305 | Lower | -3.0846 | Lower | +19.4458 | Higher |
| Cyclomatic (sum) | 19 | **17 ✅** | 19 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 31 | **18 ✅** | 39 | +13 | Higher | -8 | Lower | -21 | Lower |
| Halstead Volume | 2750.1993 | **1668.9174 ✅** | 2816.6877 | +1081.2819 | Higher | -66.4883 | Lower | -1147.7703 | Lower |
| Halstead Effort | 173262.5574 | **67619.9282 ✅** | 180564.5037 | +105642.6293 | Higher | -7301.9462 | Lower | -112944.5755 | Lower |
| SLOC | 76 | 73 | 73 | +3 | Higher | +3 | Higher | 0 | Same |
| PLOC | 63 | **50 ✅** | 62 | +13 | Higher | +1 | Higher | -12 | Lower |
| LLOC | 58 | **35 ✅** | 56 | +23 | Higher | +2 | Higher | -21 | Lower |
| Comments (CLOC) | 4 | **18 ✅** | 5 | -14 | Lower | -1 | Lower | +13 | Higher |
| Blank Lines | 13 | 12 | **9 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00799

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.7203 | **57.0979 ✅** | 49.9230 | -8.3776 | Lower | -1.2027 | Lower | +7.1749 | Higher |
| MI (SEI) | 15.6909 | **46.3773 ✅** | 13.5609 | -30.6864 | Lower | +2.1300 | Higher | +32.8164 | Higher |
| Cyclomatic (sum) | 21 | **18 ✅** | 21 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 63 | **28 ✅** | 55 | +35 | Higher | +8 | Higher | -27 | Lower |
| Halstead Volume | 2736.6375 | **1872.9564 ✅** | 2826 | +863.6811 | Higher | -89.3625 | Lower | -953.0436 | Lower |
| Halstead Effort | 125630.7526 | **73193.1642 ✅** | 177957.2571 | +52437.5884 | Higher | -52326.5045 | Lower | -104764.0930 | Lower |
| SLOC | 111 | **78 ✅** | 102 | +33 | Higher | +9 | Higher | -24 | Lower |
| PLOC | 93 | **53 ✅** | 88 | +40 | Higher | +5 | Higher | -35 | Lower |
| LLOC | 66 | **39 ✅** | 76 | +27 | Higher | -10 | Lower | -37 | Lower |
| Comments (CLOC) | 7 | **24 ✅** | 4 | -17 | Lower | +3 | Higher | +20 | Higher |
| Blank Lines | 15 | 14 | **12 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00811

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.8875 | **61.7986 ✅** | 57.2358 | -3.9112 | Lower | +0.6516 | Higher | +4.5628 | Higher |
| MI (SEI) | 25.8337 | **38.3946 ✅** | 17.8468 | -12.5609 | Lower | +7.9868 | Higher | +20.5477 | Higher |
| Cyclomatic (sum) | 20 | 19 | **18 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Cognitive (sum) | 27 | **17 ✅** | 25 | +10 | Higher | +2 | Higher | -8 | Lower |
| Halstead Volume | 2258.3879 | **1411.5155 ✅** | 2444.7075 | +846.8724 | Higher | -186.3196 | Lower | -1033.1920 | Lower |
| Halstead Effort | 128257.6145 | **47905.9801 ✅** | 153261.5886 | +80351.6344 | Higher | -25003.9741 | Lower | -105355.6085 | Lower |
| SLOC | 68 | **63 ✅** | 71 | +5 | Higher | -3 | Lower | -8 | Lower |
| PLOC | 57 | **51 ✅** | 61 | +6 | Higher | -4 | Lower | -10 | Lower |
| LLOC | 41 | **28 ✅** | 50 | +13 | Higher | -9 | Lower | -22 | Lower |
| Comments (CLOC) | 3 | **6 ✅** | 1 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | 11 | 11 | **10 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |

## p00816

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.3665 | **65.8156 ✅** | 57.4744 | -11.4491 | Lower | -3.1080 | Lower | +8.3412 | Higher |
| MI (SEI) | 28.7751 | **45.1182 ✅** | 21.4434 | -16.3431 | Lower | +7.3317 | Higher | +23.6748 | Higher |
| Cyclomatic (sum) | 22 | **14 ✅** | 16 | +8 | Higher | +6 | Higher | -2 | Lower |
| Cognitive (sum) | 28 | **14 ✅** | 26 | +14 | Higher | +2 | Higher | -12 | Lower |
| Halstead Volume | 2653.4768 | **2166.4692 ✅** | 2242.4702 | +487.0075 | Higher | +411.0065 | Higher | -76.0010 | Lower |
| Halstead Effort | 143016.3669 | **49249.6370 ✅** | 109054.8675 | +93766.7299 | Higher | +33961.4994 | Higher | -59805.2305 | Lower |
| SLOC | 78 | **46 ✅** | 74 | +32 | Higher | +4 | Higher | -28 | Lower |
| PLOC | 70 | **37 ✅** | 66 | +33 | Higher | +4 | Higher | -29 | Lower |
| LLOC | 50 | **20 ✅** | 51 | +30 | Higher | -1 | Lower | -31 | Lower |
| Comments (CLOC) | **8 ✅** | 5 | 2 | +3 | Higher | +6 | Higher | +3 | Higher |
| Blank Lines | 8 | **6 ✅** | 8 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00820

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.0160 ✅** | 66.0461 | 77.1759 | +14.9698 | Higher | +3.8401 | Higher | -11.1297 | Lower |
| MI (SEI) | 41.9951 | 36.5135 | **64.0874 ✅** | +5.4815 | Higher | -22.0924 | Lower | -27.5739 | Lower |
| Cyclomatic (sum) | 8 | 12 | 8 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 7 | 18 | 7 | -11 | Lower | 0 | Same | +11 | Higher |
| Halstead Volume | **713.1384 ✅** | 2117.4669 | 744.6997 | -1404.3286 | Lower | -31.5614 | Lower | +1372.7672 | Higher |
| Halstead Effort | **22226.1457 ✅** | 44402.6397 | 22850.5233 | -22176.4940 | Lower | -624.3776 | Lower | +21552.1165 | Higher |
| SLOC | **28 ✅** | 47 | 35 | -19 | Lower | -7 | Lower | +12 | Higher |
| PLOC | **25 ✅** | 40 | 26 | -15 | Lower | -1 | Lower | +14 | Higher |
| LLOC | 17 | 17 | 18 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 0 | 2 | **5 ✅** | -2 | Lower | -5 | Lower | -3 | Lower |
| Blank Lines | **3 ✅** | 6 | 4 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.1896 | **59.4747 ✅** | 45.5514 | -12.2851 | Lower | +1.6382 | Higher | +13.9233 | Higher |
| MI (SEI) | 7.0687 | **41.4455 ✅** | 1.8033 | -34.3768 | Lower | +5.2654 | Higher | +39.6422 | Higher |
| Cyclomatic (sum) | 20 | **14 ✅** | 19 | +6 | Higher | +1 | Higher | -5 | Lower |
| Cognitive (sum) | 44 | **31 ✅** | 41 | +13 | Higher | +3 | Higher | -10 | Lower |
| Halstead Volume | 3949.2123 | **1982.7474 ✅** | 4096.8302 | +1966.4650 | Higher | -147.6179 | Lower | -2114.0828 | Lower |
| Halstead Effort | 293953.0385 | **65117.5979 ✅** | 356424.2275 | +228835.4407 | Higher | -62471.1889 | Lower | -291306.6296 | Lower |
| SLOC | 110 | **70 ✅** | 122 | +40 | Higher | -12 | Lower | -52 | Lower |
| PLOC | 94 | **52 ✅** | 102 | +42 | Higher | -8 | Lower | -50 | Lower |
| LLOC | 84 | **39 ✅** | 105 | +45 | Higher | -21 | Lower | -66 | Lower |
| Comments (CLOC) | 3 | **12 ✅** | 2 | -9 | Lower | +1 | Higher | +10 | Higher |
| Blank Lines | 15 | **10 ✅** | 18 | +5 | Higher | -3 | Lower | -8 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p00846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.5332 | 46.8448 | **47.9180 ✅** | +0.6884 | Higher | -0.3848 | Lower | -1.0732 | Lower |
| MI (SEI) | 9.0993 | **30.0891 ✅** | 11.8459 | -20.9898 | Lower | -2.7466 | Lower | +18.2432 | Higher |
| Cyclomatic (sum) | **17 ✅** | 27 | 18 | -10 | Lower | -1 | Lower | +9 | Higher |
| Cognitive (sum) | 18 | 19 | **17 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Halstead Volume | 4103.8678 | **2927.2796 ✅** | 4335.4623 | +1176.5882 | Higher | -231.5945 | Lower | -1408.1827 | Lower |
| Halstead Effort | 218692.9544 | **187973.1705 ✅** | 229847.2431 | +30719.7839 | Higher | -11154.2887 | Lower | -41874.0726 | Lower |
| SLOC | 111 | 112 | **105 ✅** | -1 | Lower | +6 | Higher | +7 | Higher |
| PLOC | 92 | **76 ✅** | 87 | +16 | Higher | +5 | Higher | -11 | Lower |
| LLOC | 50 | **35 ✅** | 52 | +15 | Higher | -2 | Lower | -17 | Lower |
| Comments (CLOC) | 4 | **29 ✅** | 5 | -25 | Lower | -1 | Lower | +24 | Higher |
| Blank Lines | 17 | 16 | 16 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **3 ✅** | 11 | 5 | -8 | Lower | -2 | Lower | +6 | Higher |

## p00855

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.4359 | 61.0526 | **73.9648 ✅** | +11.3833 | Higher | -1.5289 | Lower | -12.9122 | Lower |
| MI (SEI) | 41.6250 | 24.2113 | **66.6562 ✅** | +17.4137 | Higher | -25.0312 | Lower | -42.4449 | Lower |
| Cyclomatic (sum) | 11 | 17 | **10 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Cognitive (sum) | **7 ✅** | 14 | 10 | -7 | Lower | -3 | Lower | +4 | Higher |
| Halstead Volume | 854.4993 | 2303 | **772.0879 ✅** | -1448.5007 | Lower | +82.4114 | Higher | +1530.9121 | Higher |
| Halstead Effort | **21225.7617 ✅** | 51099.2376 | 23250.3728 | -29873.4759 | Lower | -2024.6111 | Lower | +27848.8649 | Higher |
| SLOC | 43 | 58 | **41 ✅** | -15 | Lower | +2 | Higher | +17 | Higher |
| PLOC | 35 | 51 | **30 ✅** | -16 | Lower | +5 | Higher | +21 | Higher |
| LLOC | **19 ✅** | 22 | 20 | -3 | Lower | -1 | Lower | +2 | Higher |
| Comments (CLOC) | 1 | 1 | **10 ✅** | 0 | Same | -9 | Lower | -9 | Lower |
| Blank Lines | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 4 | 5 | **2 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 5 | 7 | **1 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |

## p00856

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.9845 | **56.5147 ✅** | 54.7581 | -0.5302 | Lower | +1.2264 | Higher | +1.7567 | Higher |
| MI (SEI) | 31.1941 | **35.3289 ✅** | 25.4633 | -4.1347 | Lower | +5.7308 | Higher | +9.8656 | Higher |
| Cyclomatic (sum) | 24 | **20 ✅** | 24 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 17 | **15 ✅** | 17 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 2606.9832 | **2570.5943 ✅** | 2769.0045 | +36.3889 | Higher | -162.0213 | Lower | -198.4102 | Lower |
| Halstead Effort | 155630.8343 | **146427.4776 ✅** | 158615.8016 | +9203.3568 | Higher | -2984.9672 | Lower | -12188.3240 | Lower |
| SLOC | **69 ✅** | 71 | 73 | -2 | Lower | -4 | Lower | -2 | Lower |
| PLOC | 54 | **48 ✅** | 58 | +6 | Higher | -4 | Lower | -10 | Lower |
| LLOC | 36 | **29 ✅** | 40 | +7 | Higher | -4 | Lower | -11 | Lower |
| Comments (CLOC) | 7 | **10 ✅** | 5 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | 10 | 14 | 10 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | **5 ✅** | 7 | +2 | Higher | 0 | Same | -2 | Lower |

## p00866

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.9001 | **61.4839 ✅** | 49.6862 | -9.5838 | Lower | +2.2139 | Higher | +11.7977 | Higher |
| MI (SEI) | 12.5544 | **40.7178 ✅** | 13.0592 | -28.1634 | Lower | -0.5048 | Lower | +27.6586 | Higher |
| Cyclomatic (sum) | 19 | **9 ✅** | 18 | +10 | Higher | +1 | Higher | -9 | Lower |
| Cognitive (sum) | 35 | **11 ✅** | 32 | +24 | Higher | +3 | Higher | -21 | Lower |
| Halstead Volume | 3117.5595 | **2716.9039 ✅** | 3592.2820 | +400.6556 | Higher | -474.7225 | Lower | -875.3781 | Lower |
| Halstead Effort | 172515.5630 | **55535.3578 ✅** | 229906.0479 | +116980.2052 | Higher | -57390.4849 | Lower | -174370.6901 | Lower |
| SLOC | 90 | **60 ✅** | 100 | +30 | Higher | -10 | Lower | -40 | Lower |
| PLOC | 75 | **43 ✅** | 84 | +32 | Higher | -9 | Lower | -41 | Lower |
| LLOC | 47 | **26 ✅** | 53 | +21 | Higher | -6 | Lower | -27 | Lower |
| Comments (CLOC) | 2 | **8 ✅** | 4 | -6 | Lower | -2 | Lower | +4 | Higher |
| Blank Lines | 13 | **10 ✅** | 13 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00867

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 8.9722 | **62.1450 ✅** | 11.5438 | -53.1727 | Lower | -2.5716 | Lower | +50.6011 | Higher |
| MI (SEI) | -45.4582 | **48.9537 ✅** | -44.3510 | -94.4119 | Lower | -1.1072 | Lower | +93.3047 | Higher |
| Cyclomatic (sum) | 85 | **9 ✅** | 81 | +76 | Higher | +4 | Higher | -72 | Lower |
| Cognitive (sum) | 106 | **6 ✅** | 94 | +100 | Higher | +12 | Higher | -88 | Lower |
| Halstead Volume | 12689.9240 | **1956.7918 ✅** | 12714.4067 | +10733.1323 | Higher | -24.4827 | Lower | -10757.6150 | Lower |
| Halstead Effort | 1838474.4729 | **37590.4146 ✅** | 2010443.7939 | +1800884.0583 | Higher | -171969.3210 | Lower | -1972853.3793 | Lower |
| SLOC | 318 | **64 ✅** | 287 | +254 | Higher | +31 | Higher | -223 | Lower |
| PLOC | 274 | **39 ✅** | 252 | +235 | Higher | +22 | Higher | -213 | Lower |
| LLOC | 174 | **13 ✅** | 160 | +161 | Higher | +14 | Higher | -147 | Lower |
| Comments (CLOC) | 4 | **15 ✅** | 2 | -11 | Lower | +2 | Higher | +13 | Higher |
| Blank Lines | 41 | **10 ✅** | 33 | +31 | Higher | +8 | Higher | -23 | Lower |
| Functions | 16 | **4 ✅** | 16 | +12 | Higher | 0 | Same | -12 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 18 | **3 ✅** | 16 | +15 | Higher | +2 | Higher | -13 | Lower |

## p00870

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.4280 | **58.7369 ✅** | 51.8063 | -4.3089 | Lower | +2.6217 | Higher | +6.9306 | Higher |
| MI (SEI) | 15.3236 | **44.9533 ✅** | 20.8540 | -29.6297 | Lower | -5.5304 | Lower | +24.0993 | Higher |
| Cyclomatic (sum) | 29 | **18 ✅** | 22 | +11 | Higher | +7 | Higher | -4 | Lower |
| Cognitive (sum) | 68 | **10 ✅** | 31 | +58 | Higher | +37 | Higher | -21 | Lower |
| Halstead Volume | 3395.3980 | **2531.0105 ✅** | 2878.2919 | +864.3875 | Higher | +517.1061 | Higher | -347.2814 | Lower |
| Halstead Effort | 113346.3733 | **57502.7820 ✅** | 129218.7008 | +55843.5913 | Higher | -15872.3276 | Lower | -71715.9189 | Lower |
| SLOC | 65 | **64 ✅** | 89 | +1 | Higher | -24 | Lower | -25 | Lower |
| PLOC | 63 | **38 ✅** | 72 | +25 | Higher | -9 | Lower | -34 | Lower |
| LLOC | 49 | **17 ✅** | 52 | +32 | Higher | -3 | Lower | -35 | Lower |
| Comments (CLOC) | 1 | **15 ✅** | 6 | -14 | Lower | -5 | Lower | +9 | Higher |
| Blank Lines | **2 ✅** | 13 | 12 | -11 | Lower | -10 | Lower | +1 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | **0 ✅** | 3 | 2 | -3 | Lower | -2 | Lower | +1 | Higher |
| Exits | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |

## p00874

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.7575 | 67.9740 | **72.0491 ✅** | -2.2165 | Lower | -6.2916 | Lower | -4.0752 | Lower |
| MI (SEI) | 51.7557 | **55.4203 ✅** | 49.5055 | -3.6647 | Lower | +2.2502 | Higher | +5.9149 | Higher |
| Cyclomatic (sum) | 9 | 11 | 9 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | **9 ✅** | 17 | 11 | -8 | Lower | -2 | Lower | +6 | Higher |
| Halstead Volume | 1981.7103 | 1259.7972 | **1166.4637 ✅** | +721.9131 | Higher | +815.2467 | Higher | +93.3335 | Higher |
| Halstead Effort | 39560.8099 | 38966.8299 | **31090.7429 ✅** | +593.9799 | Higher | +8470.0669 | Higher | +7876.0870 | Higher |
| SLOC | 51 | 50 | **41 ✅** | +1 | Higher | +10 | Higher | +9 | Higher |
| PLOC | 37 | 34 | **32 ✅** | +3 | Higher | +5 | Higher | +2 | Higher |
| LLOC | **24 ✅** | 27 | 25 | -3 | Lower | -1 | Lower | +2 | Higher |
| Comments (CLOC) | 10 | 10 | 3 | 0 | Same | +7 | Higher | +7 | Higher |
| Blank Lines | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00889

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.1778 | **71.4881 ✅** | 46.4687 | -23.3103 | Lower | +1.7091 | Higher | +25.0194 | Higher |
| MI (SEI) | -3.9550 | **58.2496 ✅** | 3.7672 | -62.2046 | Lower | -7.7222 | Lower | +54.4823 | Higher |
| Cyclomatic (sum) | 22 | **7 ✅** | 22 | +15 | Higher | 0 | Same | -15 | Lower |
| Cognitive (sum) | 40 | **7 ✅** | 40 | +33 | Higher | 0 | Same | -33 | Lower |
| Halstead Volume | 3668.8004 | **991.8765 ✅** | 3715.1521 | +2676.9239 | Higher | -46.3517 | Lower | -2723.2756 | Lower |
| Halstead Effort | 315183.3067 | **26355.5759 ✅** | 276818.3545 | +288827.7308 | Higher | +38364.9522 | Higher | -250462.7786 | Lower |
| SLOC | 103 | **46 ✅** | 114 | +57 | Higher | -11 | Lower | -68 | Lower |
| PLOC | 94 | **29 ✅** | 99 | +65 | Higher | -5 | Lower | -70 | Lower |
| LLOC | 85 | **22 ✅** | 78 | +63 | Higher | +7 | Higher | -56 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 2 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | **9 ✅** | 11 | 13 | -2 | Lower | -4 | Lower | -2 | Lower |
| Functions | 3 | **1 ✅** | 4 | +2 | Higher | -1 | Lower | -3 | Lower |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 3 | **0 ✅** | 4 | +3 | Higher | -1 | Lower | -4 | Lower |

## p00897

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 32.5459 | **49.7752 ✅** | 21.3140 | -17.2292 | Lower | +11.2319 | Higher | +28.4611 | Higher |
| MI (SEI) | -24.9797 | **20.3031 ✅** | -33.4169 | -45.2828 | Lower | +8.4373 | Higher | +53.7200 | Higher |
| Cyclomatic (sum) | 37 | **21 ✅** | 48 | +16 | Higher | -11 | Lower | -27 | Lower |
| Cognitive (sum) | 34 | **18 ✅** | 65 | +16 | Higher | -31 | Lower | -47 | Lower |
| Halstead Volume | 6159.9655 | **4009.8658 ✅** | 10111.6878 | +2150.0997 | Higher | -3951.7223 | Lower | -6101.8220 | Lower |
| Halstead Effort | 321557.2810 | **95622.1293 ✅** | 1065890.8585 | +225935.1518 | Higher | -744333.5775 | Lower | -970268.7293 | Lower |
| SLOC | 185 | **92 ✅** | 270 | +93 | Higher | -85 | Lower | -178 | Lower |
| PLOC | 157 | **73 ✅** | 229 | +84 | Higher | -72 | Lower | -156 | Lower |
| LLOC | 83 | **29 ✅** | 184 | +54 | Higher | -101 | Lower | -155 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 2 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | 28 | **16 ✅** | 39 | +12 | Higher | -11 | Lower | -23 | Lower |
| Functions | 13 | **5 ✅** | 12 | +8 | Higher | +1 | Higher | -7 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 11 | **3 ✅** | 9 | +8 | Higher | +2 | Higher | -6 | Lower |

## p00907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.1369 | 59.1095 | **61.8490 ✅** | -3.9726 | Lower | -6.7121 | Lower | -2.7394 | Lower |
| MI (SEI) | 21.4305 | **38.7360 ✅** | 32.3050 | -17.3055 | Lower | -10.8745 | Lower | +6.4310 | Higher |
| Cyclomatic (sum) | 20 | **15 ✅** | 19 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 35 | **14 ✅** | 35 | +21 | Higher | 0 | Same | -21 | Lower |
| Halstead Volume | 3207.6224 | 2825.6011 | **1545.7159 ✅** | +382.0213 | Higher | +1661.9065 | Higher | +1279.8852 | Higher |
| Halstead Effort | 90203.9192 | 75025.8505 | **71635.9347 ✅** | +15178.0687 | Higher | +18567.9845 | Higher | +3389.9158 | Higher |
| SLOC | 72 | 63 | **61 ✅** | +9 | Higher | +11 | Higher | +2 | Higher |
| PLOC | 59 | **45 ✅** | 49 | +14 | Higher | +10 | Higher | -4 | Lower |
| LLOC | 38 | **25 ✅** | 36 | +13 | Higher | +2 | Higher | -11 | Lower |
| Comments (CLOC) | 3 | **9 ✅** | 3 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | 11 | 11 | **10 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00924

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.2772 | **68.2090 ✅** | 51.5475 | -17.9318 | Lower | -1.2702 | Lower | +16.6615 | Higher |
| MI (SEI) | 19.2730 | **57.5865 ✅** | 20.6982 | -38.3136 | Lower | -1.4253 | Lower | +36.8883 | Higher |
| Cyclomatic (sum) | 25 | **12 ✅** | 21 | +13 | Higher | +4 | Higher | -9 | Lower |
| Cognitive (sum) | 25 | **9 ✅** | 25 | +16 | Higher | 0 | Same | -16 | Lower |
| Halstead Volume | 3763.6543 | **1715.6197 ✅** | 3518.5101 | +2048.0345 | Higher | +245.1442 | Higher | -1802.8904 | Lower |
| Halstead Effort | 233517.6387 | **37636.4075 ✅** | 239087.0513 | +195881.2312 | Higher | -5569.4125 | Lower | -201450.6438 | Lower |
| SLOC | 86 | **44 ✅** | 86 | +42 | Higher | 0 | Same | -42 | Lower |
| PLOC | 71 | **30 ✅** | 72 | +41 | Higher | -1 | Lower | -42 | Lower |
| LLOC | 52 | **16 ✅** | 61 | +36 | Higher | -9 | Lower | -45 | Lower |
| Comments (CLOC) | 6 | **10 ✅** | 6 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 10 | **6 ✅** | 9 | +4 | Higher | +1 | Higher | -3 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 6 | **0 ✅** | 2 | +6 | Higher | +4 | Higher | -2 | Lower |
| Exits | 8 | **2 ✅** | 8 | +6 | Higher | 0 | Same | -6 | Lower |

## p00949

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.6227 | **58.8924 ✅** | 40.4339 | -13.2697 | Lower | +5.1887 | Higher | +18.4585 | Higher |
| MI (SEI) | 10.6787 | **37.6086 ✅** | 4.8752 | -26.9300 | Lower | +5.8035 | Higher | +32.7334 | Higher |
| Cyclomatic (sum) | 21 | **12 ✅** | 26 | +9 | Higher | -5 | Lower | -14 | Lower |
| Cognitive (sum) | 22 | **14 ✅** | 31 | +8 | Higher | -9 | Lower | -17 | Lower |
| Halstead Volume | 4103.7613 | **2125.8958 ✅** | 4405.5509 | +1977.8654 | Higher | -301.7897 | Lower | -2279.6551 | Lower |
| Halstead Effort | 290581.2233 | **90630.2963 ✅** | 391543.3396 | +199950.9271 | Higher | -100962.1163 | Lower | -300913.0434 | Lower |
| SLOC | 118 | **73 ✅** | 148 | +45 | Higher | -30 | Lower | -75 | Lower |
| PLOC | 100 | **49 ✅** | 121 | +51 | Higher | -21 | Lower | -72 | Lower |
| LLOC | 80 | **38 ✅** | 103 | +42 | Higher | -23 | Lower | -65 | Lower |
| Comments (CLOC) | 7 | 10 | 10 | -3 | Lower | -3 | Lower | 0 | Same |
| Blank Lines | 17 | **14 ✅** | 18 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 5 | **3 ✅** | 4 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |
| Exits | 7 | **2 ✅** | 6 | +5 | Higher | +1 | Higher | -4 | Lower |

## p00971

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.0143 | **58.7333 ✅** | 50.9629 | -5.7190 | Lower | +2.0514 | Higher | +7.7704 | Higher |
| MI (SEI) | 13.9466 | **37.7247 ✅** | -0.5479 | -23.7781 | Lower | +14.4945 | Higher | +38.2725 | Higher |
| Cyclomatic (sum) | **15 ✅** | 17 | 16 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | **13 ✅** | 22 | 14 | -9 | Lower | -1 | Lower | +8 | Higher |
| Halstead Volume | 3337.8348 | **1684.1329 ✅** | 3849.0485 | +1653.7018 | Higher | -511.2138 | Lower | -2164.9156 | Lower |
| Halstead Effort | 273648.6136 | **59863.2704 ✅** | 393152.8143 | +213785.3431 | Higher | -119504.2008 | Lower | -333289.5439 | Lower |
| SLOC | 87 | **74 ✅** | 93 | +13 | Higher | -6 | Lower | -19 | Lower |
| PLOC | 74 | **53 ✅** | 79 | +21 | Higher | -5 | Lower | -26 | Lower |
| LLOC | 55 | **39 ✅** | 74 | +16 | Higher | -19 | Lower | -35 | Lower |
| Comments (CLOC) | 2 | **10 ✅** | 0 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 12 | **11 ✅** | 14 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 4 | 3 | 3 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |
| Exits | 3 | 2 | 2 | +1 | Higher | +1 | Higher | 0 | Same |

## p01050

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.5717 ✅** | 69.0475 | 72.5168 | +3.5242 | Higher | +0.0549 | Higher | -3.4693 | Lower |
| MI (SEI) | 69.7062 | 57.9808 | **70.7773 ✅** | +11.7254 | Higher | -1.0711 | Lower | -12.7964 | Lower |
| Cyclomatic (sum) | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 14 | **5 ✅** | 13 | +9 | Higher | +1 | Higher | -8 | Lower |
| Halstead Volume | **825.0630 ✅** | 1698.3711 | 894.2888 | -873.3081 | Lower | -69.2258 | Lower | +804.0823 | Higher |
| Halstead Effort | 24690.0105 | 29056.4700 | **22752.7709 ✅** | -4366.4595 | Lower | +1937.2396 | Higher | +6303.6991 | Higher |
| SLOC | 45 | 45 | **44 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| PLOC | 33 | **27 ✅** | 33 | +6 | Higher | 0 | Same | -6 | Lower |
| LLOC | 27 | **14 ✅** | 31 | +13 | Higher | -4 | Lower | -17 | Lower |
| Comments (CLOC) | 16 | 10 | **17 ✅** | +6 | Higher | -1 | Lower | -7 | Lower |
| Blank Lines | 6 | 8 | 6 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p01065

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.7854 | **66.1549 ✅** | 59.0852 | -12.3695 | Lower | -5.2998 | Lower | +7.0697 | Higher |
| MI (SEI) | 24.4545 | **48.6992 ✅** | 24.6927 | -24.2446 | Lower | -0.2382 | Lower | +24.0065 | Higher |
| Cyclomatic (sum) | 17 | **8 ✅** | 16 | +9 | Higher | +1 | Higher | -8 | Lower |
| Cognitive (sum) | 30 | **6 ✅** | 31 | +24 | Higher | -1 | Lower | -25 | Lower |
| Halstead Volume | 3701.5918 | **2474.9632 ✅** | 2586 | +1226.6286 | Higher | +1115.5918 | Higher | -111.0368 | Lower |
| Halstead Effort | 136092.5678 | **45412.1692 ✅** | 166063.1351 | +90680.3986 | Higher | -29970.5674 | Lower | -120650.9659 | Lower |
| SLOC | 78 | **47 ✅** | 64 | +31 | Higher | +14 | Higher | -17 | Lower |
| PLOC | 62 | **31 ✅** | 53 | +31 | Higher | +9 | Higher | -22 | Lower |
| LLOC | 34 | **12 ✅** | 40 | +22 | Higher | -6 | Lower | -28 | Lower |
| Comments (CLOC) | 6 | **7 ✅** | 2 | -1 | Lower | +4 | Higher | +5 | Higher |
| Blank Lines | 10 | 10 | **9 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p01106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.7253 | **72.3294 ✅** | 68.9098 | -4.6041 | Lower | -1.1845 | Lower | +3.4195 | Higher |
| MI (SEI) | 55.6080 | **65.6442 ✅** | 59.8945 | -10.0362 | Lower | -4.2866 | Lower | +5.7496 | Higher |
| Cyclomatic (sum) | 11 | **8 ✅** | 9 | +3 | Higher | +2 | Higher | -1 | Lower |
| Cognitive (sum) | 17 | **9 ✅** | 17 | +8 | Higher | 0 | Same | -8 | Lower |
| Halstead Volume | 1500.7152 | **927.1038 ✅** | 1394.0325 | +573.6114 | Higher | +106.6827 | Higher | -466.9287 | Lower |
| Halstead Effort | 55267.7169 | **18210.9670 ✅** | 52758.7677 | +37056.7499 | Higher | +2508.9492 | Higher | -34547.8007 | Lower |
| SLOC | 48 | **44 ✅** | 47 | +4 | Higher | +1 | Higher | -3 | Lower |
| PLOC | 35 | **27 ✅** | 37 | +8 | Higher | -2 | Lower | -10 | Lower |
| LLOC | 25 | **17 ✅** | 31 | +8 | Higher | -6 | Lower | -14 | Lower |
| Comments (CLOC) | 10 | 12 | 12 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 8 | 9 | 8 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01119

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.7522 | **61.5789 ✅** | 32.1791 | -13.8268 | Lower | +15.5730 | Higher | +29.3998 | Higher |
| MI (SEI) | 6.2945 | **50.2476 ✅** | -17.1251 | -43.9531 | Lower | +23.4196 | Higher | +67.3727 | Higher |
| Cyclomatic (sum) | 23 | **14 ✅** | 41 | +9 | Higher | -18 | Lower | -27 | Lower |
| Cognitive (sum) | 59 | **12 ✅** | 67 | +47 | Higher | -8 | Lower | -55 | Lower |
| Halstead Volume | 3927 | **2508.9618 ✅** | 5355.8444 | +1418.0382 | Higher | -1428.8444 | Lower | -2846.8825 | Lower |
| Halstead Effort | 112825.7308 | **50537.6600 ✅** | 388730.6395 | +62288.0707 | Higher | -275904.9087 | Lower | -338192.9794 | Lower |
| SLOC | 102 | **57 ✅** | 187 | +45 | Higher | -85 | Lower | -130 | Lower |
| PLOC | 85 | **41 ✅** | 165 | +44 | Higher | -80 | Lower | -124 | Lower |
| LLOC | 69 | **23 ✅** | 123 | +46 | Higher | -54 | Lower | -100 | Lower |
| Comments (CLOC) | 2 | **15 ✅** | 2 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | 15 | **10 ✅** | 20 | +5 | Higher | -5 | Lower | -10 | Lower |
| Functions | **1 ✅** | 2 | 9 | -1 | Lower | -8 | Lower | -7 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 4 | 10 | -4 | Lower | -10 | Lower | -6 | Lower |

## p01125

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.0678 | 56.6719 | **60.5075 ✅** | +2.3959 | Higher | -1.4397 | Lower | -3.8355 | Lower |
| MI (SEI) | 36.6844 | **37.7810 ✅** | 22.8174 | -1.0966 | Lower | +13.8670 | Higher | +14.9636 | Higher |
| Cyclomatic (sum) | 18 | 18 | **15 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Cognitive (sum) | 28 | 20 | **19 ✅** | +8 | Higher | +9 | Higher | +1 | Higher |
| Halstead Volume | **1966.2505 ✅** | 2204.1866 | 2159.5278 | -237.9361 | Lower | -193.2773 | Lower | +44.6589 | Higher |
| Halstead Effort | **61468.7358 ✅** | 88167.4648 | 71716.4100 | -26698.7290 | Lower | -10247.6742 | Lower | +16451.0548 | Higher |
| SLOC | 68 | 76 | **63 ✅** | -8 | Lower | +5 | Higher | +13 | Higher |
| PLOC | 51 | 51 | 53 | 0 | Same | -2 | Lower | -2 | Lower |
| LLOC | **31 ✅** | 33 | 43 | -2 | Lower | -12 | Lower | -10 | Lower |
| Comments (CLOC) | 8 | **13 ✅** | 1 | -5 | Lower | +7 | Higher | +12 | Higher |
| Blank Lines | 11 | 13 | **10 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p01128

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.0060 | **48.3632 ✅** | 46.0800 | -3.3572 | Lower | -1.0741 | Lower | +2.2831 | Higher |
| MI (SEI) | -8.3273 | **15.9607 ✅** | -6.5741 | -24.2880 | Lower | -1.7532 | Lower | +22.5348 | Higher |
| Cyclomatic (sum) | 24 | **14 ✅** | 26 | +10 | Higher | -2 | Lower | -12 | Lower |
| Cognitive (sum) | 41 | **16 ✅** | 37 | +25 | Higher | +4 | Higher | -21 | Lower |
| Halstead Volume | 4046.3369 | 4350.8769 | **3969.6291 ✅** | -304.5399 | Lower | +76.7078 | Higher | +381.2477 | Higher |
| Halstead Effort | 228388.9985 | **93291.2209 ✅** | 247949.1419 | +135097.7777 | Higher | -19560.1434 | Lower | -154657.9211 | Lower |
| SLOC | 118 | 108 | 108 | +10 | Higher | +10 | Higher | 0 | Same |
| PLOC | 100 | **82 ✅** | 97 | +18 | Higher | +3 | Higher | -15 | Lower |
| LLOC | 57 | **24 ✅** | 62 | +33 | Higher | -5 | Lower | -38 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 0 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 18 | 20 | **11 ✅** | -2 | Lower | +7 | Higher | +9 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **1 ✅** | 2 | 3 | -1 | Lower | -2 | Lower | -1 | Lower |
| Exits | 1 | 1 | 5 | 0 | Same | -4 | Lower | -4 | Lower |

## p01138

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.1443 ✅** | 61.0883 | 66.8211 | +7.0561 | Higher | +1.3232 | Higher | -5.7329 | Lower |
| MI (SEI) | 44.8409 | 47.0233 | **47.9993 ✅** | -2.1824 | Lower | -3.1584 | Lower | -0.9760 | Lower |
| Cyclomatic (sum) | 8 | 17 | **6 ✅** | -9 | Lower | +2 | Higher | +11 | Higher |
| Cognitive (sum) | 4 | 26 | 4 | -22 | Lower | 0 | Same | +22 | Higher |
| Halstead Volume | **1482.6295 ✅** | 2551.4866 | 1636.0262 | -1068.8571 | Lower | -153.3967 | Lower | +915.4604 | Higher |
| Halstead Effort | **44478.8862 ✅** | 53443.3009 | 55794.1349 | -8964.4147 | Lower | -11315.2487 | Lower | -2350.8340 | Lower |
| SLOC | **49 ✅** | 56 | 53 | -7 | Lower | -4 | Lower | +3 | Higher |
| PLOC | **36 ✅** | 41 | 37 | -5 | Lower | -1 | Lower | +4 | Higher |
| LLOC | **24 ✅** | 29 | 33 | -5 | Lower | -9 | Lower | -4 | Lower |
| Comments (CLOC) | 4 | **12 ✅** | 7 | -8 | Lower | -3 | Lower | +5 | Higher |
| Blank Lines | 9 | **5 ✅** | 10 | +4 | Higher | -1 | Lower | -5 | Lower |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p01150

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 24.1488 | **74.6640 ✅** | 26.1050 | -50.5151 | Lower | -1.9562 | Lower | +48.5590 | Higher |
| MI (SEI) | -18.3925 | **68.3570 ✅** | -16.3909 | -86.7495 | Lower | -2.0017 | Lower | +84.7479 | Higher |
| Cyclomatic (sum) | 60 | **10 ✅** | 56 | +50 | Higher | +4 | Higher | -46 | Lower |
| Cognitive (sum) | 77 | **9 ✅** | 63 | +68 | Higher | +14 | Higher | -54 | Lower |
| Halstead Volume | 6909.8674 | **788.7516 ✅** | 6662.7145 | +6121.1158 | Higher | +247.1529 | Higher | -5873.9629 | Lower |
| Halstead Effort | 1087152.4721 | **22407.7164 ✅** | 974355.3686 | +1064744.7558 | Higher | +112797.1035 | Higher | -951947.6522 | Lower |
| SLOC | 216 | **39 ✅** | 205 | +177 | Higher | +11 | Higher | -166 | Lower |
| PLOC | 192 | **28 ✅** | 181 | +164 | Higher | +11 | Higher | -153 | Lower |
| LLOC | 144 | **11 ✅** | 146 | +133 | Higher | -2 | Lower | -135 | Lower |
| Comments (CLOC) | 10 | 10 | 9 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | 17 | **6 ✅** | 19 | +11 | Higher | -2 | Lower | -13 | Lower |
| Functions | 6 | **2 ✅** | 6 | +4 | Higher | 0 | Same | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 18 | **1 ✅** | 6 | +17 | Higher | +12 | Higher | -5 | Lower |

## p01153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.2552 | **68.4041 ✅** | 67.1756 | -0.1489 | Lower | +1.0797 | Higher | +1.2286 | Higher |
| MI (SEI) | 53.3452 | **59.8012 ✅** | 37.6801 | -6.4560 | Lower | +15.6651 | Higher | +22.1211 | Higher |
| Cyclomatic (sum) | 10 | **8 ✅** | 12 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 20 | **5 ✅** | 32 | +15 | Higher | -12 | Lower | -27 | Lower |
| Halstead Volume | 1416.6079 | **1172.0182 ✅** | 1405.3110 | +244.5897 | Higher | +11.2969 | Higher | -233.2927 | Lower |
| Halstead Effort | 56228.4368 | **42192.6569 ✅** | 58741.9989 | +14035.7799 | Higher | -2513.5622 | Lower | -16549.3420 | Lower |
| SLOC | **48 ✅** | 52 | 50 | -4 | Lower | -2 | Lower | +2 | Higher |
| PLOC | 34 | **31 ✅** | 41 | +3 | Higher | -7 | Lower | -10 | Lower |
| LLOC | 25 | **21 ✅** | 33 | +4 | Higher | -8 | Lower | -12 | Lower |
| Comments (CLOC) | 8 | **14 ✅** | 2 | -6 | Lower | +6 | Higher | +12 | Higher |
| Blank Lines | 7 | 11 | 7 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p01184

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.8911 | **58.5564 ✅** | 33.9548 | -23.6653 | Lower | +0.9363 | Higher | +24.6016 | Higher |
| MI (SEI) | -11.6990 | **29.5352 ✅** | -14.7273 | -41.2341 | Lower | +3.0283 | Higher | +44.2625 | Higher |
| Cyclomatic (sum) | 35 | **17 ✅** | 37 | +18 | Higher | -2 | Lower | -20 | Lower |
| Cognitive (sum) | 62 | **25 ✅** | 58 | +37 | Higher | +4 | Higher | -33 | Lower |
| Halstead Volume | 5188.8517 | **2876.6700 ✅** | 5487.6601 | +2312.1817 | Higher | -298.8084 | Lower | -2610.9901 | Lower |
| Halstead Effort | 426638.9163 | **59118.2322 ✅** | 464256.0469 | +367520.6840 | Higher | -37617.1307 | Lower | -405137.8147 | Lower |
| SLOC | 174 | **63 ✅** | 176 | +111 | Higher | -2 | Lower | -113 | Lower |
| PLOC | 146 | **53 ✅** | 149 | +93 | Higher | -3 | Lower | -96 | Lower |
| LLOC | 105 | **27 ✅** | 109 | +78 | Higher | -4 | Lower | -82 | Lower |
| Comments (CLOC) | 3 | **4 ✅** | 2 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 26 | **8 ✅** | 25 | +18 | Higher | +1 | Higher | -17 | Lower |
| Functions | 5 | **1 ✅** | 5 | +4 | Higher | 0 | Same | -4 | Lower |
| Closures | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |
| Exits | 7 | **0 ✅** | 9 | +7 | Higher | -2 | Lower | -9 | Lower |

## p01255

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 32.3509 | **47.3485 ✅** | 11.7340 | -14.9976 | Lower | +20.6169 | Higher | +35.6144 | Higher |
| MI (SEI) | -25.1592 | **6.8659 ✅** | -42.3766 | -32.0251 | Lower | +17.2174 | Higher | +49.2425 | Higher |
| Cyclomatic (sum) | 38 | **30 ✅** | 76 | +8 | Higher | -38 | Lower | -46 | Lower |
| Cognitive (sum) | 58 | **28 ✅** | 115 | +30 | Higher | -57 | Lower | -87 | Lower |
| Halstead Volume | 6329.4427 | **4016.2748 ✅** | 11218.7082 | +2313.1679 | Higher | -4889.2655 | Lower | -7202.4333 | Lower |
| Halstead Effort | 360094.9422 | **246659.8983 ✅** | 1304471.7906 | +113435.0439 | Higher | -944376.8484 | Lower | -1057811.8923 | Lower |
| SLOC | 183 | **94 ✅** | 317 | +89 | Higher | -134 | Lower | -223 | Lower |
| PLOC | 161 | **77 ✅** | 284 | +84 | Higher | -123 | Lower | -207 | Lower |
| LLOC | 85 | **43 ✅** | 162 | +42 | Higher | -77 | Lower | -119 | Lower |
| Comments (CLOC) | 0 | 2 | **4 ✅** | -2 | Lower | -4 | Lower | -2 | Lower |
| Blank Lines | 22 | **15 ✅** | 29 | +7 | Higher | -7 | Lower | -14 | Lower |
| Functions | 10 | **5 ✅** | 17 | +5 | Higher | -7 | Lower | -12 | Lower |
| Closures | 2 | **0 ✅** | 3 | +2 | Higher | -1 | Lower | -3 | Lower |
| Exits | 12 | **4 ✅** | 26 | +8 | Higher | -14 | Lower | -22 | Lower |

## p01275

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.7942 | **68.7483 ✅** | 50.0721 | -8.9540 | Lower | +9.7221 | Higher | +18.6762 | Higher |
| MI (SEI) | 12.1931 | **53.7942 ✅** | 20.5803 | -41.6012 | Lower | -8.3872 | Lower | +33.2139 | Higher |
| Cyclomatic (sum) | 16 | **10 ✅** | 20 | +6 | Higher | -4 | Lower | -10 | Lower |
| Cognitive (sum) | 14 | **7 ✅** | 20 | +7 | Higher | -6 | Lower | -13 | Lower |
| Halstead Volume | 2149.9853 | **1208.3042 ✅** | 2701.8442 | +941.6811 | Higher | -551.8589 | Lower | -1493.5400 | Lower |
| Halstead Effort | 92739.9069 | **36832.4447 ✅** | 144335.3597 | +55907.4622 | Higher | -51595.4528 | Lower | -107502.9150 | Lower |
| SLOC | 65 | **49 ✅** | 104 | +16 | Higher | -39 | Lower | -55 | Lower |
| PLOC | 54 | **31 ✅** | 83 | +23 | Higher | -29 | Lower | -52 | Lower |
| LLOC | 34 | **20 ✅** | 55 | +14 | Higher | -21 | Lower | -35 | Lower |
| Comments (CLOC) | 0 | 8 | **9 ✅** | -8 | Lower | -9 | Lower | -1 | Lower |
| Blank Lines | 11 | 11 | 16 | 0 | Same | -5 | Lower | -5 | Lower |
| Functions | 5 | **2 ✅** | 6 | +3 | Higher | -1 | Lower | -4 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 4 | **1 ✅** | 5 | +3 | Higher | -1 | Lower | -4 | Lower |

## p01288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.6947 | 60.5687 | **62.5028 ✅** | +0.1260 | Higher | -1.8081 | Lower | -1.9341 | Lower |
| MI (SEI) | 33.3750 | **41.6378 ✅** | 25.4444 | -8.2628 | Lower | +7.9306 | Higher | +16.1934 | Higher |
| Cyclomatic (sum) | 11 | **10 ✅** | 11 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 22 | **5 ✅** | 22 | +17 | Higher | 0 | Same | -17 | Lower |
| Halstead Volume | 1959.8510 | 2662.4840 | **1941.7650 ✅** | -702.6329 | Lower | +18.0860 | Higher | +720.7189 | Higher |
| Halstead Effort | 81244.7342 | **58408.2419 ✅** | 92401.2327 | +22836.4923 | Higher | -11156.4984 | Lower | -33992.9908 | Lower |
| SLOC | 68 | 63 | **61 ✅** | +5 | Higher | +7 | Higher | +2 | Higher |
| PLOC | 54 | **46 ✅** | 53 | +8 | Higher | +1 | Higher | -7 | Lower |
| LLOC | 49 | **26 ✅** | 50 | +23 | Higher | -1 | Lower | -24 | Lower |
| Comments (CLOC) | 5 | **10 ✅** | 1 | -5 | Lower | +4 | Higher | +9 | Higher |
| Blank Lines | 10 | 9 | **8 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01316

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.0158 | **63.3220 ✅** | 58.0665 | -8.3061 | Lower | -3.0507 | Lower | +5.2554 | Higher |
| MI (SEI) | 23.5788 | **54.4102 ✅** | 18.9435 | -30.8314 | Lower | +4.6353 | Higher | +35.4668 | Higher |
| Cyclomatic (sum) | 19 | **13 ✅** | 17 | +6 | Higher | +2 | Higher | -4 | Lower |
| Cognitive (sum) | 34 | **12 ✅** | 35 | +22 | Higher | -1 | Lower | -23 | Lower |
| Halstead Volume | 3584.5062 | **1875.5492 ✅** | 2178 | +1708.9570 | Higher | +1406.5062 | Higher | -302.4508 | Lower |
| Halstead Effort | 100753.6878 | **37564.2669 ✅** | 84942 | +63189.4209 | Higher | +15811.6878 | Higher | -47377.7331 | Lower |
| SLOC | 71 | **57 ✅** | 71 | +14 | Higher | 0 | Same | -14 | Lower |
| PLOC | 58 | **39 ✅** | 58 | +19 | Higher | 0 | Same | -19 | Lower |
| LLOC | 34 | **24 ✅** | 36 | +10 | Higher | -2 | Lower | -12 | Lower |
| Comments (CLOC) | 4 | **17 ✅** | 1 | -13 | Lower | +3 | Higher | +16 | Higher |
| Blank Lines | 13 | **11 ✅** | 12 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **2 ✅** | 4 | 3 | -2 | Lower | -1 | Lower | +1 | Higher |

## p01317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **53.1399 ✅** | 51.7192 | 50.0279 | +1.4206 | Higher | +3.1120 | Higher | +1.6913 | Higher |
| MI (SEI) | 2.6947 | **23.7971 ✅** | 6.3225 | -21.1024 | Lower | -3.6278 | Lower | +17.4746 | Higher |
| Cyclomatic (sum) | **17 ✅** | 19 | 19 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | 38 | **29 ✅** | 41 | +9 | Higher | -3 | Lower | -12 | Lower |
| Halstead Volume | 3726 | **2727.5309 ✅** | 3775.9447 | +998.4691 | Higher | -49.9447 | Lower | -1048.4138 | Lower |
| Halstead Effort | 230302.2857 | **177257.7958 ✅** | 207165.6344 | +53044.4899 | Higher | +23136.6513 | Higher | -29907.8386 | Lower |
| SLOC | **81 ✅** | 95 | 95 | -14 | Lower | -14 | Lower | 0 | Same |
| PLOC | **69 ✅** | 75 | 79 | -6 | Lower | -10 | Lower | -4 | Lower |
| LLOC | 61 | **41 ✅** | 68 | +20 | Higher | -7 | Lower | -27 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 1 | -9 | Lower | -1 | Lower | +8 | Higher |
| Blank Lines | 12 | 12 | 16 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p01339

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 35.9791 | **48.7836 ✅** | 34.7866 | -12.8045 | Lower | +1.1924 | Higher | +13.9969 | Higher |
| MI (SEI) | -12.0491 | **11.6046 ✅** | -13.7624 | -23.6537 | Lower | +1.7133 | Higher | +25.3670 | Higher |
| Cyclomatic (sum) | 32 | **20 ✅** | 34 | +12 | Higher | -2 | Lower | -14 | Lower |
| Cognitive (sum) | 31 | **23 ✅** | 31 | +8 | Higher | 0 | Same | -8 | Lower |
| Halstead Volume | 5671.4418 | **3359.9151 ✅** | 5633.8411 | +2311.5267 | Higher | +37.6007 | Higher | -2273.9260 | Lower |
| Halstead Effort | 445263.7852 | **163989.7043 ✅** | 459544.6858 | +281274.0809 | Higher | -14280.9006 | Lower | -295554.9815 | Lower |
| SLOC | 165 | **105 ✅** | 173 | +60 | Higher | -8 | Lower | -68 | Lower |
| PLOC | 145 | **87 ✅** | 154 | +58 | Higher | -9 | Lower | -67 | Lower |
| LLOC | 98 | **51 ✅** | 102 | +47 | Higher | -4 | Lower | -51 | Lower |
| Comments (CLOC) | 2 | **4 ✅** | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 18 | 17 | 17 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 8 | **6 ✅** | 10 | +2 | Higher | -2 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **3 ✅** | 5 | +1 | Higher | -1 | Lower | -2 | Lower |

## p01353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.4637 | **61.3433 ✅** | 55.9734 | -8.8796 | Lower | -3.5097 | Lower | +5.3699 | Higher |
| MI (SEI) | 13.5580 | **46.7063 ✅** | 18.3549 | -33.1483 | Lower | -4.7969 | Lower | +28.3514 | Higher |
| Cyclomatic (sum) | 17 | **11 ✅** | 13 | +6 | Higher | +4 | Higher | -2 | Lower |
| Cognitive (sum) | 11 | **4 ✅** | 12 | +7 | Higher | -1 | Lower | -8 | Lower |
| Halstead Volume | 3788.8435 | **2426.8093 ✅** | 2482.1196 | +1362.0343 | Higher | +1306.7239 | Higher | -55.3104 | Lower |
| Halstead Effort | 138859.3451 | **41636.8269 ✅** | 144326.1766 | +97222.5182 | Higher | -5466.8315 | Lower | -102689.3498 | Lower |
| SLOC | 84 | **61 ✅** | 82 | +23 | Higher | +2 | Higher | -21 | Lower |
| PLOC | 70 | **37 ✅** | 68 | +33 | Higher | +2 | Higher | -31 | Lower |
| LLOC | 47 | **19 ✅** | 54 | +28 | Higher | -7 | Lower | -35 | Lower |
| Comments (CLOC) | 2 | **13 ✅** | 2 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 12 | **11 ✅** | 12 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |

## p01368

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.8365 | **53.9931 ✅** | 44.2756 | -15.1566 | Lower | -5.4391 | Lower | +9.7176 | Higher |
| MI (SEI) | 3.6227 | **25.1411 ✅** | 2.8564 | -21.5184 | Lower | +0.7663 | Higher | +22.2847 | Higher |
| Cyclomatic (sum) | 30 | **13 ✅** | 27 | +17 | Higher | +3 | Higher | -14 | Lower |
| Cognitive (sum) | 46 | **15 ✅** | 53 | +31 | Higher | -7 | Lower | -38 | Lower |
| Halstead Volume | 4618.4601 | **3369.8648 ✅** | 3494.0760 | +1248.5953 | Higher | +1124.3841 | Higher | -124.2112 | Lower |
| Halstead Effort | 424109.8104 | **92814.0715 ✅** | 264719.9321 | +331295.7389 | Higher | +159389.8783 | Higher | -171905.8606 | Lower |
| SLOC | 152 | **84 ✅** | 124 | +68 | Higher | +28 | Higher | -40 | Lower |
| PLOC | 126 | **64 ✅** | 104 | +62 | Higher | +22 | Higher | -40 | Lower |
| LLOC | 77 | **30 ✅** | 69 | +47 | Higher | +8 | Higher | -39 | Lower |
| Comments (CLOC) | **11 ✅** | 7 | 3 | +4 | Higher | +8 | Higher | +4 | Higher |
| Blank Lines | 17 | **14 ✅** | 17 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 7 | **2 ✅** | 5 | +5 | Higher | +2 | Higher | -3 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 6 | **1 ✅** | 4 | +5 | Higher | +2 | Higher | -3 | Lower |

## p01452

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.6703 | **75.0922 ✅** | 54.7163 | -19.4220 | Lower | +0.9539 | Higher | +20.3759 | Higher |
| MI (SEI) | 15.0383 | **81.9425 ✅** | 17.0421 | -66.9042 | Lower | -2.0037 | Lower | +64.9004 | Higher |
| Cyclomatic (sum) | 15 | 16 | 15 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 7 | 10 | **6 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Halstead Volume | 3180.1951 | **2528.1693 ✅** | 3381.1189 | +652.0258 | Higher | -200.9238 | Lower | -852.9496 | Lower |
| Halstead Effort | 176235.8120 | **70214.1566 ✅** | 219465.3541 | +106021.6554 | Higher | -43229.5421 | Lower | -149251.1975 | Lower |
| SLOC | 75 | **24 ✅** | 78 | +51 | Higher | -3 | Lower | -54 | Lower |
| PLOC | 60 | **43 ✅** | 65 | +17 | Higher | -5 | Lower | -22 | Lower |
| LLOC | 33 | **18 ✅** | 39 | +15 | Higher | -6 | Lower | -21 | Lower |
| Comments (CLOC) | 1 | **16 ✅** | 2 | -15 | Lower | -1 | Lower | +14 | Higher |
| Blank Lines | 15 | **-28 ✅** | 13 | +43 | Higher | +2 | Higher | -41 | Lower |
| Functions | 6 | **5 ✅** | 7 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |

## p01459

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.2475 | **47.2170 ✅** | 38.7407 | -5.9695 | Lower | +2.5068 | Higher | +8.4763 | Higher |
| MI (SEI) | 0.3861 | **18.2740 ✅** | -7.3463 | -17.8878 | Lower | +7.7324 | Higher | +25.6202 | Higher |
| Cyclomatic (sum) | 29 | **27 ✅** | 32 | +2 | Higher | -3 | Lower | -5 | Lower |
| Cognitive (sum) | 50 | **27 ✅** | 62 | +23 | Higher | -12 | Lower | -35 | Lower |
| Halstead Volume | 5447.1193 | **3432.8070 ✅** | 5563.5181 | +2014.3123 | Higher | -116.3988 | Lower | -2130.7111 | Lower |
| Halstead Effort | 324227.3959 | **203982.7741 ✅** | 419224.7699 | +120244.6218 | Higher | -94997.3740 | Lower | -215241.9958 | Lower |
| SLOC | 126 | **104 ✅** | 140 | +22 | Higher | -14 | Lower | -36 | Lower |
| PLOC | 107 | **84 ✅** | 118 | +23 | Higher | -11 | Lower | -34 | Lower |
| LLOC | 71 | **41 ✅** | 98 | +30 | Higher | -27 | Lower | -57 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 2 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 19 | **16 ✅** | 20 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 7 | 1 | -6 | Lower | 0 | Same | +6 | Higher |

## p01471

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.9541 | **60.6578 ✅** | 55.6972 | -0.7037 | Lower | +4.2569 | Higher | +4.9606 | Higher |
| MI (SEI) | 34.2755 | **42.6899 ✅** | 20.7317 | -8.4144 | Lower | +13.5438 | Higher | +21.9582 | Higher |
| Cyclomatic (sum) | 14 | **13 ✅** | 17 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | 22 | **10 ✅** | 19 | +12 | Higher | +3 | Higher | -9 | Lower |
| Halstead Volume | 1891.0235 | **1512 ✅** | 1960.8364 | +379.0235 | Higher | -69.8129 | Lower | -448.8364 | Lower |
| Halstead Effort | 65165.6649 | **44100 ✅** | 83874.7749 | +21065.6649 | Higher | -18709.1100 | Lower | -39774.7749 | Lower |
| SLOC | **69 ✅** | 72 | 85 | -3 | Lower | -16 | Lower | -13 | Lower |
| PLOC | 60 | **56 ✅** | 70 | +4 | Higher | -10 | Lower | -14 | Lower |
| LLOC | 33 | **27 ✅** | 43 | +6 | Higher | -10 | Lower | -16 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 3 | -6 | Lower | +3 | Higher | +9 | Higher |
| Blank Lines | **8 ✅** | 10 | 12 | -2 | Lower | -4 | Lower | -2 | Lower |
| Functions | **1 ✅** | 3 | 4 | -2 | Lower | -3 | Lower | -1 | Lower |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **0 ✅** | 2 | 3 | -2 | Lower | -3 | Lower | -1 | Lower |

## p01521

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **84.4134 ✅** | 72.5678 | 80.6528 | +11.8456 | Higher | +3.7606 | Higher | -8.0850 | Lower |
| MI (SEI) | 74.7715 | 57.6430 | **75.2883 ✅** | +17.1284 | Higher | -0.5168 | Lower | -17.6452 | Lower |
| Cyclomatic (sum) | 6 | 10 | 6 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | **453.9955 ✅** | 1653.5594 | 673.8746 | -1199.5639 | Lower | -219.8791 | Lower | +979.6848 | Higher |
| Halstead Effort | **5864.1082 ✅** | 23976.6111 | 12736.2292 | -18112.5029 | Lower | -6872.1210 | Lower | +11240.3819 | Higher |
| SLOC | **27 ✅** | 35 | 30 | -8 | Lower | -3 | Lower | +5 | Higher |
| PLOC | **19 ✅** | 26 | 21 | -7 | Lower | -2 | Lower | +5 | Higher |
| LLOC | 13 | **12 ✅** | 14 | +1 | Higher | -1 | Lower | -2 | Lower |
| Comments (CLOC) | 4 | 5 | **7 ✅** | -1 | Lower | -3 | Lower | -2 | Lower |
| Blank Lines | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01529

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.6057 | **68.5842 ✅** | 63.0002 | -3.9785 | Lower | +1.6054 | Higher | +5.5840 | Higher |
| MI (SEI) | 40.6686 | **59.7785 ✅** | 33.5636 | -19.1099 | Lower | +7.1050 | Higher | +26.2149 | Higher |
| Cyclomatic (sum) | 10 | **9 ✅** | 11 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 16 | **4 ✅** | 17 | +12 | Higher | -1 | Lower | -13 | Lower |
| Halstead Volume | **1673.1835 ✅** | 2106.9442 | 2064.8431 | -433.7606 | Lower | -391.6595 | Lower | +42.1011 | Higher |
| Halstead Effort | 85847.1859 | **49930.1869 ✅** | 119494.4664 | +35916.9990 | Higher | -33647.2805 | Lower | -69564.2795 | Lower |
| SLOC | 57 | **42 ✅** | 58 | +15 | Higher | -1 | Lower | -16 | Lower |
| PLOC | 43 | **31 ✅** | 45 | +12 | Higher | -2 | Lower | -14 | Lower |
| LLOC | 35 | **19 ✅** | 40 | +16 | Higher | -5 | Lower | -21 | Lower |
| Comments (CLOC) | 5 | **11 ✅** | 3 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 9 | **6 ✅** | 11 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p01545

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.4109 | **98.5183 ✅** | 51.4522 | -44.1074 | Lower | +2.9587 | Higher | +47.0661 | Higher |
| MI (SEI) | 21.0172 | **116.8266 ✅** | 21.1825 | -95.8094 | Lower | -0.1653 | Lower | +95.6441 | Higher |
| Cyclomatic (sum) | 17 | **4 ✅** | 20 | +13 | Higher | -3 | Lower | -16 | Lower |
| Cognitive (sum) | 13 | **0 ✅** | 15 | +13 | Higher | -2 | Lower | -15 | Lower |
| Halstead Volume | 2511.1595 | **2207.4880 ✅** | 2935.2828 | +303.6715 | Higher | -424.1234 | Lower | -727.7949 | Lower |
| Halstead Effort | 162894.9502 | **37765.1715 ✅** | 190526.5405 | +125129.7787 | Higher | -27631.5903 | Lower | -152761.3690 | Lower |
| SLOC | 85 | **7 ✅** | 93 | +78 | Higher | -8 | Lower | -86 | Lower |
| PLOC | 70 | **22 ✅** | 77 | +48 | Higher | -7 | Lower | -55 | Lower |
| LLOC | 40 | **9 ✅** | 55 | +31 | Higher | -15 | Lower | -46 | Lower |
| Comments (CLOC) | 4 | 7 | 7 | -3 | Lower | -3 | Lower | 0 | Same |
| Blank Lines | 11 | **-19 ✅** | 12 | +30 | Higher | -1 | Lower | -31 | Lower |
| Functions | 4 | **3 ✅** | 5 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 4 | **1 ✅** | 6 | +3 | Higher | -2 | Lower | -5 | Lower |

## p01570

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.7695 | **58.2564 ✅** | 37.8308 | -6.4869 | Lower | +13.9387 | Higher | +20.4256 | Higher |
| MI (SEI) | 11.2777 | **42.2187 ✅** | -9.6753 | -30.9410 | Lower | +20.9530 | Higher | +51.8940 | Higher |
| Cyclomatic (sum) | 29 | **17 ✅** | 27 | +12 | Higher | +2 | Higher | -10 | Lower |
| Cognitive (sum) | 38 | **10 ✅** | 39 | +28 | Higher | -1 | Lower | -29 | Lower |
| Halstead Volume | 4919.0260 | **2901.6182 ✅** | 5785.3316 | +2017.4078 | Higher | -866.3056 | Lower | -2883.7134 | Lower |
| Halstead Effort | 160814.3121 | **73628.5615 ✅** | 291992.3611 | +87185.7506 | Higher | -131178.0490 | Lower | -218363.7996 | Lower |
| SLOC | 68 | **64 ✅** | 157 | +4 | Higher | -89 | Lower | -93 | Lower |
| PLOC | 83 | **44 ✅** | 135 | +39 | Higher | -52 | Lower | -91 | Lower |
| LLOC | 71 | **23 ✅** | 101 | +48 | Higher | -30 | Lower | -78 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 2 | -12 | Lower | -1 | Lower | +11 | Higher |
| Blank Lines | **-15 ✅** | 12 | 20 | -27 | Lower | -35 | Lower | -8 | Lower |
| Functions | 3 | 3 | 4 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | **0 ✅** | 3 | 2 | -3 | Lower | -2 | Lower | +1 | Higher |

## p01600

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.0630 | 59.7739 | **66.0298 ✅** | +1.2891 | Higher | -4.9667 | Lower | -6.2559 | Lower |
| MI (SEI) | 40.8230 | **44.8971 ✅** | 36.1289 | -4.0741 | Lower | +4.6941 | Higher | +8.7682 | Higher |
| Cyclomatic (sum) | 12 | 12 | 13 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | **12 ✅** | 17 | 21 | -5 | Lower | -9 | Lower | -4 | Lower |
| Halstead Volume | 2867.2585 | 2703.6116 | **1675.9293 ✅** | +163.6469 | Higher | +1191.3292 | Higher | +1027.6823 | Higher |
| Halstead Effort | **64953.3409 ✅** | 72997.5134 | 76428.1541 | -8044.1726 | Lower | -11474.8132 | Lower | -3430.6407 | Lower |
| SLOC | 58 | 64 | **50 ✅** | -6 | Lower | +8 | Higher | +14 | Higher |
| PLOC | 41 | 41 | **40 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| LLOC | 24 | **23 ✅** | 30 | +1 | Higher | -6 | Lower | -7 | Lower |
| Comments (CLOC) | 8 | **14 ✅** | 2 | -6 | Lower | +6 | Higher | +12 | Higher |
| Blank Lines | 10 | 10 | **8 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |

## p01601

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.9073 | 71.1441 | **75.5549 ✅** | -1.2367 | Lower | -5.6476 | Lower | -4.4109 | Lower |
| MI (SEI) | 41.7215 | **62.9279 ✅** | 46.7135 | -21.2064 | Lower | -4.9920 | Lower | +16.2144 | Higher |
| Cyclomatic (sum) | 10 | 10 | **8 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 6 | 8 | **5 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Halstead Volume | 1031.0257 | **812.7943 ✅** | 855.4067 | +218.2314 | Higher | +175.6190 | Higher | -42.6124 | Lower |
| Halstead Effort | 35332.4570 | **20213.8398 ✅** | 28189.5384 | +15118.6172 | Higher | +7142.9186 | Higher | -7975.6986 | Lower |
| SLOC | 48 | 48 | **37 ✅** | 0 | Same | +11 | Higher | +11 | Higher |
| PLOC | 38 | 30 | 30 | +8 | Higher | +8 | Higher | 0 | Same |
| LLOC | 27 | **17 ✅** | 22 | +10 | Higher | +5 | Higher | -5 | Lower |
| Comments (CLOC) | 2 | **12 ✅** | 1 | -10 | Lower | +1 | Higher | +11 | Higher |
| Blank Lines | 9 | 9 | **7 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 3 | **1 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |

## p01656

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.6059 | 62.2410 | **75.9386 ✅** | +9.3650 | Higher | -4.3327 | Lower | -13.6977 | Lower |
| MI (SEI) | 58.5002 | 46.2420 | **64.1797 ✅** | +12.2582 | Higher | -5.6795 | Lower | -17.9377 | Lower |
| Cyclomatic (sum) | 5 | 9 | **4 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Cognitive (sum) | 4 | 6 | **3 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Halstead Volume | 1134.4151 | 2610.4545 | **1127.5815 ✅** | -1476.0394 | Lower | +6.8336 | Higher | +1482.8730 | Higher |
| Halstead Effort | 21553.8868 | 48305.2740 | **20996.3459 ✅** | -26751.3872 | Lower | +557.5409 | Higher | +27308.9281 | Higher |
| SLOC | 45 | 58 | **35 ✅** | -13 | Lower | +10 | Higher | +23 | Higher |
| PLOC | 29 | 39 | **24 ✅** | -10 | Lower | +5 | Higher | +15 | Higher |
| LLOC | 22 | 27 | **20 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| Comments (CLOC) | 8 | **11 ✅** | 6 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | 8 | 10 | **6 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p01658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.9752 | 65.9698 | **68.9810 ✅** | +1.0054 | Higher | -2.0058 | Lower | -3.0112 | Lower |
| MI (SEI) | 54.2713 | 55.0762 | **56.4791 ✅** | -0.8049 | Lower | -2.2078 | Lower | -1.4029 | Lower |
| Cyclomatic (sum) | 9 | 9 | 10 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 13 | **7 ✅** | 13 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | **1390.9073 ✅** | 2154.9499 | 1406.7615 | -764.0426 | Lower | -15.8542 | Lower | +748.1884 | Higher |
| Halstead Effort | 57027.1989 | **39089.0133 ✅** | 59984.3097 | +17938.1857 | Higher | -2957.1108 | Lower | -20895.2964 | Lower |
| SLOC | 53 | 49 | **46 ✅** | +4 | Higher | +7 | Higher | +3 | Higher |
| PLOC | 40 | **34 ✅** | 37 | +6 | Higher | +3 | Higher | -3 | Lower |
| LLOC | 38 | **19 ✅** | 37 | +19 | Higher | +1 | Higher | -18 | Lower |
| Comments (CLOC) | 11 | **12 ✅** | 9 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 8 | 8 | **7 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.9539 | **60.8570 ✅** | 43.5822 | -17.9031 | Lower | -0.6282 | Lower | +17.2748 | Higher |
| MI (SEI) | -10.0659 | **48.6729 ✅** | -9.3632 | -58.7388 | Lower | -0.7027 | Lower | +58.0361 | Higher |
| Cyclomatic (sum) | 36 | **18 ✅** | 34 | +18 | Higher | +2 | Higher | -16 | Lower |
| Cognitive (sum) | 30 | **14 ✅** | 34 | +16 | Higher | -4 | Lower | -20 | Lower |
| Halstead Volume | 4789.9616 | **1955.1808 ✅** | 5221.9126 | +2834.7808 | Higher | -431.9510 | Lower | -3266.7318 | Lower |
| Halstead Effort | 442273.1186 | **88960.7258 ✅** | 485637.8724 | +353312.3928 | Higher | -43364.7538 | Lower | -396677.1466 | Lower |
| SLOC | 107 | **61 ✅** | 103 | +46 | Higher | +4 | Higher | -42 | Lower |
| PLOC | 88 | **37 ✅** | 84 | +51 | Higher | +4 | Higher | -47 | Lower |
| LLOC | 57 | **25 ✅** | 68 | +32 | Higher | -11 | Lower | -43 | Lower |
| Comments (CLOC) | 0 | **15 ✅** | 0 | -15 | Lower | 0 | Same | +15 | Higher |
| Blank Lines | 19 | **12 ✅** | 19 | +7 | Higher | 0 | Same | -7 | Lower |
| Functions | 5 | **2 ✅** | 4 | +3 | Higher | +1 | Higher | -2 | Lower |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | 9 | **3 ✅** | 8 | +6 | Higher | +1 | Higher | -5 | Lower |

## p01725

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 36.7253 | **36.9704 ✅** | 36.9068 | -0.2451 | Lower | -0.1815 | Lower | +0.0636 | Higher |
| MI (SEI) | **0.7651 ✅** | -9.4357 | -4.0709 | +10.2008 | Higher | +4.8360 | Higher | -5.3648 | Lower |
| Cyclomatic (sum) | 40 | 41 | **35 ✅** | -1 | Lower | +5 | Higher | +6 | Higher |
| Cognitive (sum) | **37 ✅** | 43 | 50 | -6 | Lower | -13 | Lower | -7 | Lower |
| Halstead Volume | 4546.3401 | **3824.7665 ✅** | 4663.3704 | +721.5736 | Higher | -117.0304 | Lower | -838.6039 | Lower |
| Halstead Effort | 253699.5533 | **216246.4135 ✅** | 263236.8210 | +37453.1398 | Higher | -9537.2677 | Lower | -46990.4075 | Lower |
| SLOC | **151 ✅** | 155 | 159 | -4 | Lower | -8 | Lower | -4 | Lower |
| PLOC | **130 ✅** | 137 | 134 | -7 | Lower | -4 | Lower | +3 | Higher |
| LLOC | **49 ✅** | 59 | 60 | -10 | Lower | -11 | Lower | -1 | Lower |
| Comments (CLOC) | **10 ✅** | 2 | 6 | +8 | Higher | +4 | Higher | -4 | Lower |
| Blank Lines | 19 | **16 ✅** | 22 | +3 | Higher | -3 | Lower | -6 | Lower |
| Functions | 9 | **7 ✅** | 8 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |

## p01767

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.5848 | 58.4578 | **60.8588 ✅** | +0.1270 | Higher | -2.2740 | Lower | -2.4010 | Lower |
| MI (SEI) | 34.2735 | **40.1486 ✅** | 22.8703 | -5.8750 | Lower | +11.4032 | Higher | +17.2783 | Higher |
| Cyclomatic (sum) | 12 | 12 | 12 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 12 | **7 ✅** | 12 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | **2073.3186 ✅** | 2882.9644 | 2091.0393 | -809.6458 | Lower | -17.7207 | Lower | +791.9251 | Higher |
| Halstead Effort | 90372.2989 | **56610.9371 ✅** | 92743.7418 | +33761.3618 | Higher | -2371.4428 | Lower | -36132.8047 | Lower |
| SLOC | 75 | 68 | **65 ✅** | +7 | Higher | +10 | Higher | +3 | Higher |
| PLOC | 52 | **45 ✅** | 52 | +7 | Higher | 0 | Same | -7 | Lower |
| LLOC | 36 | **24 ✅** | 36 | +12 | Higher | 0 | Same | -12 | Lower |
| Comments (CLOC) | 8 | **12 ✅** | 1 | -4 | Lower | +7 | Higher | +11 | Higher |
| Blank Lines | 15 | 12 | 12 | +3 | Higher | +3 | Higher | 0 | Same |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |

## p01768

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.2320 | **55.4778 ✅** | 48.3017 | -4.2459 | Lower | +2.9303 | Higher | +7.1761 | Higher |
| MI (SEI) | -0.1597 | **30.2040 ✅** | 9.7401 | -30.3636 | Lower | -9.8998 | Lower | +20.4639 | Higher |
| Cyclomatic (sum) | 16 | **14 ✅** | 16 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | **8 ✅** | 9 | 11 | -1 | Lower | -3 | Lower | -2 | Lower |
| Halstead Volume | 3205.5551 | **1825.2292 ✅** | 3140.4918 | +1380.3259 | Higher | +65.0633 | Higher | -1315.2626 | Lower |
| Halstead Effort | 107910.6423 | **46087.0371 ✅** | 147001.7450 | +61823.6051 | Higher | -39091.1027 | Lower | -100914.7079 | Lower |
| SLOC | 97 | **92 ✅** | 117 | +5 | Higher | -20 | Lower | -25 | Lower |
| PLOC | 80 | **70 ✅** | 99 | +10 | Higher | -19 | Lower | -29 | Lower |
| LLOC | 40 | **28 ✅** | 63 | +12 | Higher | -23 | Lower | -35 | Lower |
| Comments (CLOC) | 0 | **10 ✅** | 4 | -10 | Lower | -4 | Lower | +6 | Higher |
| Blank Lines | 17 | **16 ✅** | 18 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p01782

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 26.2448 | **63.2881 ✅** | 25.1978 | -37.0433 | Lower | +1.0470 | Higher | +38.0903 | Higher |
| MI (SEI) | -26.0806 | **43.9738 ✅** | -28.0531 | -70.0544 | Lower | +1.9726 | Higher | +72.0270 | Higher |
| Cyclomatic (sum) | 44 | **12 ✅** | 42 | +32 | Higher | +2 | Higher | -30 | Lower |
| Cognitive (sum) | 51 | **14 ✅** | 53 | +37 | Higher | -2 | Lower | -39 | Lower |
| Halstead Volume | 8251.5556 | **2790.3090 ✅** | 8787.0966 | +5461.2465 | Higher | -535.5410 | Lower | -5996.7875 | Lower |
| Halstead Effort | 758703.0314 | **59712.6137 ✅** | 848442.9899 | +698990.4178 | Higher | -89739.9585 | Lower | -788730.3762 | Lower |
| SLOC | 225 | **51 ✅** | 242 | +174 | Higher | -17 | Lower | -191 | Lower |
| PLOC | 195 | **40 ✅** | 211 | +155 | Higher | -16 | Lower | -171 | Lower |
| LLOC | 133 | **18 ✅** | 156 | +115 | Higher | -23 | Lower | -138 | Lower |
| Comments (CLOC) | 2 | **7 ✅** | 2 | -5 | Lower | 0 | Same | +5 | Higher |
| Blank Lines | 28 | **7 ✅** | 29 | +21 | Higher | -1 | Lower | -22 | Lower |
| Functions | 13 | **1 ✅** | 12 | +12 | Higher | +1 | Higher | -11 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 5 | **1 ✅** | 4 | +4 | Higher | +1 | Higher | -3 | Lower |

## p01841

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.1133 | **54.6097 ✅** | 46.6356 | -10.4964 | Lower | -2.5223 | Lower | +7.9741 | Higher |
| MI (SEI) | 7.5673 | **25.0326 ✅** | 9.6234 | -17.4654 | Lower | -2.0561 | Lower | +15.4092 | Higher |
| Cyclomatic (sum) | 20 | **18 ✅** | 20 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 16 | **7 ✅** | 16 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | 4240.4872 | **2399.2042 ✅** | 4039.9446 | +1841.2830 | Higher | +200.5426 | Higher | -1640.7404 | Lower |
| Halstead Effort | 269349.4639 | **81716.8939 ✅** | 228256.8705 | +187632.5701 | Higher | +41092.5934 | Higher | -146539.9767 | Lower |
| SLOC | 130 | **84 ✅** | 113 | +46 | Higher | +17 | Higher | -29 | Lower |
| PLOC | 105 | **63 ✅** | 92 | +42 | Higher | +13 | Higher | -29 | Lower |
| LLOC | 72 | **27 ✅** | 75 | +45 | Higher | -3 | Lower | -48 | Lower |
| Comments (CLOC) | **7 ✅** | 6 | 5 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 20 | **15 ✅** | 16 | +5 | Higher | +4 | Higher | -1 | Lower |
| Functions | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |

## p01847

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.2862 | **64.4971 ✅** | 54.4284 | -9.2110 | Lower | +0.8577 | Higher | +10.0687 | Higher |
| MI (SEI) | 5.9948 | **47.3845 ✅** | 17.3155 | -41.3897 | Lower | -11.3207 | Lower | +30.0690 | Higher |
| Cyclomatic (sum) | 19 | **7 ✅** | 21 | +12 | Higher | -2 | Lower | -14 | Lower |
| Cognitive (sum) | 46 | **4 ✅** | 50 | +42 | Higher | -4 | Lower | -46 | Lower |
| Halstead Volume | 4076.6838 | **2180.5267 ✅** | 2853.0282 | +1896.1571 | Higher | +1223.6557 | Higher | -672.5015 | Lower |
| Halstead Effort | 112056.5399 | **53937.1794 ✅** | 173219.5667 | +58119.3606 | Higher | -61163.0268 | Lower | -119282.3873 | Lower |
| SLOC | 67 | **55 ✅** | 77 | +12 | Higher | -10 | Lower | -22 | Lower |
| PLOC | 57 | **40 ✅** | 63 | +17 | Higher | -6 | Lower | -23 | Lower |
| LLOC | 66 | **29 ✅** | 79 | +37 | Higher | -13 | Lower | -50 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 2 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | 10 | **9 ✅** | 12 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | **1 ✅** | 3 | 2 | -2 | Lower | -1 | Lower | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |

## p01907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.4313 | **62.5929 ✅** | 38.6781 | -13.1615 | Lower | +10.7532 | Higher | +23.9147 | Higher |
| MI (SEI) | 20.7664 | **41.5552 ✅** | -8.4253 | -20.7887 | Lower | +29.1918 | Higher | +49.9805 | Higher |
| Cyclomatic (sum) | 15 | **9 ✅** | 27 | +6 | Higher | -12 | Lower | -18 | Lower |
| Cognitive (sum) | 17 | **1 ✅** | 28 | +16 | Higher | -11 | Lower | -27 | Lower |
| Halstead Volume | 3293.7245 | **2721.4761 ✅** | 5014.2605 | +572.2484 | Higher | -1720.5359 | Lower | -2292.7843 | Lower |
| Halstead Effort | 167793.5142 | **48616.4499 ✅** | 375260.7837 | +119177.0643 | Higher | -207467.2695 | Lower | -326644.3338 | Lower |
| SLOC | 109 | **56 ✅** | 156 | +53 | Higher | -47 | Lower | -100 | Lower |
| PLOC | 81 | **39 ✅** | 129 | +42 | Higher | -48 | Lower | -90 | Lower |
| LLOC | 68 | **11 ✅** | 117 | +57 | Higher | -49 | Lower | -106 | Lower |
| Comments (CLOC) | **11 ✅** | 7 | 2 | +4 | Higher | +9 | Higher | +5 | Higher |
| Blank Lines | 18 | **10 ✅** | 25 | +8 | Higher | -7 | Lower | -15 | Lower |
| Functions | 3 | 3 | 5 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |

## p01916

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.8270 | 83.4029 | **85.3195 ✅** | -7.5758 | Lower | -9.4925 | Lower | -1.9167 | Lower |
| MI (SEI) | 74.3345 | **81.3722 ✅** | 79.8338 | -7.0376 | Lower | -5.4992 | Lower | +1.5384 | Higher |
| Cyclomatic (sum) | 11 | **4 ✅** | 5 | +7 | Higher | +6 | Higher | -1 | Lower |
| Cognitive (sum) | 6 | **3 ✅** | 4 | +3 | Higher | +2 | Higher | -1 | Lower |
| Halstead Volume | 2133.9589 | **482.1526 ✅** | 506.6527 | +1651.8063 | Higher | +1627.3063 | Higher | -24.5000 | Lower |
| Halstead Effort | 34817.2247 | **5464.3962 ✅** | 7706.4535 | +29352.8286 | Higher | +27110.7713 | Higher | -2242.0573 | Lower |
| SLOC | 26 | 29 | **25 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| PLOC | 24 | **16 ✅** | 17 | +8 | Higher | +7 | Higher | -1 | Lower |
| LLOC | 11 | **10 ✅** | 13 | +1 | Higher | -2 | Lower | -3 | Lower |
| Comments (CLOC) | **9 ✅** | 8 | 5 | +1 | Higher | +4 | Higher | +3 | Higher |
| Blank Lines | **-5 ✅** | 6 | 5 | -11 | Lower | -10 | Lower | +1 | Higher |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01917

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.9032 | **58.9337 ✅** | 55.4936 | -5.0305 | Lower | -1.5903 | Lower | +3.4401 | Higher |
| MI (SEI) | 4.3051 | **48.5670 ✅** | 6.4977 | -44.2619 | Lower | -2.1926 | Lower | +42.0694 | Higher |
| Cyclomatic (sum) | 22 | **15 ✅** | 21 | +7 | Higher | +1 | Higher | -6 | Lower |
| Cognitive (sum) | 47 | **25 ✅** | 36 | +22 | Higher | +11 | Higher | -11 | Lower |
| Halstead Volume | 2680.6933 | **2201.4614 ✅** | 2744.8845 | +479.2319 | Higher | -64.1912 | Lower | -543.4231 | Lower |
| Halstead Effort | 174245.0630 | **51011.3139 ✅** | 173461.4510 | +123233.7491 | Higher | +783.6120 | Higher | -122450.1371 | Lower |
| SLOC | 80 | **69 ✅** | 73 | +11 | Higher | +7 | Higher | -4 | Lower |
| PLOC | 72 | **39 ✅** | 65 | +33 | Higher | +7 | Higher | -26 | Lower |
| LLOC | 56 | **20 ✅** | 52 | +36 | Higher | +4 | Higher | -32 | Lower |
| Comments (CLOC) | 0 | **21 ✅** | 0 | -21 | Lower | 0 | Same | +21 | Higher |
| Blank Lines | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02035

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.3758 | 54.7392 | **57.8552 ✅** | +1.6366 | Higher | -1.4794 | Lower | -3.1160 | Lower |
| MI (SEI) | 7.3632 | 21.9792 | **25.8535 ✅** | -14.6160 | Lower | -18.4904 | Lower | -3.8744 | Lower |
| Cyclomatic (sum) | 17 | 16 | 16 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 14 | **7 ✅** | 14 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | **3014.8463 ✅** | 3095.2992 | 3276.0616 | -80.4528 | Lower | -261.2153 | Lower | -180.7624 | Lower |
| Halstead Effort | 249227.2964 | **116164.7573 ✅** | 303222.9027 | +133062.5391 | Higher | -53995.6063 | Lower | -187058.1454 | Lower |
| SLOC | 71 | 79 | **64 ✅** | -8 | Lower | +7 | Higher | +15 | Higher |
| PLOC | 61 | 61 | **55 ✅** | 0 | Same | +6 | Higher | +6 | Higher |
| LLOC | **41 ✅** | 44 | 42 | -3 | Lower | -1 | Lower | +2 | Higher |
| Comments (CLOC) | 0 | **4 ✅** | 3 | -4 | Lower | -3 | Lower | +1 | Higher |
| Blank Lines | 10 | 17 | **9 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| Functions | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 6 | **4 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p02038

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.5032 | 58.6483 | **73.4267 ✅** | +5.8550 | Higher | -8.9234 | Lower | -14.7784 | Lower |
| MI (SEI) | 41.5017 | 38.9984 | **55.7329 ✅** | +2.5033 | Higher | -14.2312 | Lower | -16.7345 | Lower |
| Cyclomatic (sum) | 16 | 17 | **11 ✅** | -1 | Lower | +5 | Higher | +6 | Higher |
| Cognitive (sum) | 9 | 12 | **8 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Halstead Volume | 1462.7434 | 2444.9903 | **1127.9580 ✅** | -982.2469 | Lower | +334.7854 | Higher | +1317.0323 | Higher |
| Halstead Effort | 45041.7930 | 46035.3318 | **28872.0850 ✅** | -993.5388 | Lower | +16169.7081 | Higher | +17163.2468 | Higher |
| SLOC | 55 | 66 | **37 ✅** | -11 | Lower | +18 | Higher | +29 | Higher |
| PLOC | 39 | 49 | **31 ✅** | -10 | Lower | +8 | Higher | +18 | Higher |
| LLOC | 22 | 28 | **15 ✅** | -6 | Lower | +7 | Higher | +13 | Higher |
| Comments (CLOC) | 5 | **10 ✅** | 4 | -5 | Lower | +1 | Higher | +6 | Higher |
| Blank Lines | 11 | 11 | **6 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 8 | 5 | **4 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |

## p02045

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.1703 | **61.0745 ✅** | 59.1501 | -6.9042 | Lower | -4.9798 | Lower | +1.9244 | Higher |
| MI (SEI) | 12.6937 | **41.0737 ✅** | 11.3657 | -28.3800 | Lower | +1.3280 | Higher | +29.7080 | Higher |
| Cyclomatic (sum) | 17 | **12 ✅** | 17 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 18 | **10 ✅** | 13 | +8 | Higher | +5 | Higher | -3 | Lower |
| Halstead Volume | 2941.5682 | **2105.3407 ✅** | 2443.4098 | +836.2275 | Higher | +498.1585 | Higher | -338.0691 | Lower |
| Halstead Effort | 142489.5655 | **74608.0117 ✅** | 117641.2417 | +67881.5538 | Higher | +24848.3237 | Higher | -43033.2301 | Lower |
| SLOC | 82 | 64 | 64 | +18 | Higher | +18 | Higher | 0 | Same |
| PLOC | 69 | **42 ✅** | 54 | +27 | Higher | +15 | Higher | -12 | Lower |
| LLOC | 50 | **31 ✅** | 49 | +19 | Higher | +1 | Higher | -18 | Lower |
| Comments (CLOC) | 1 | **9 ✅** | 0 | -8 | Lower | +1 | Higher | +9 | Higher |
| Blank Lines | 13 | 13 | **10 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p02046

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.3760 ✅** | 72.4837 | 76.8203 | +5.8923 | Higher | +1.5557 | Higher | -4.3367 | Lower |
| MI (SEI) | 38.5936 | **64.1498 ✅** | 36.3493 | -25.5562 | Lower | +2.2443 | Higher | +27.8005 | Higher |
| Cyclomatic (sum) | 12 | 21 | 12 | -9 | Lower | 0 | Same | +9 | Higher |
| Cognitive (sum) | 11 | **5 ✅** | 11 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | **889.9074 ✅** | 1365.8132 | 975.0745 | -475.9058 | Lower | -85.1670 | Lower | +390.7387 | Higher |
| Halstead Effort | **28900.8029 ✅** | 69542.6553 | 37822.6253 | -40641.8525 | Lower | -8921.8224 | Lower | +31720.0300 | Higher |
| SLOC | **29 ✅** | 32 | 31 | -3 | Lower | -2 | Lower | +1 | Higher |
| PLOC | **25 ✅** | 26 | 26 | -1 | Lower | -1 | Lower | 0 | Same |
| LLOC | **10 ✅** | 11 | 21 | -1 | Lower | -11 | Lower | -10 | Lower |
| Comments (CLOC) | 0 | **7 ✅** | 0 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02056

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.2400 | **51.7373 ✅** | 21.6548 | -9.4972 | Lower | +20.5852 | Higher | +30.0825 | Higher |
| MI (SEI) | -2.6015 | **23.3940 ✅** | -40.2850 | -25.9954 | Lower | +37.6835 | Higher | +63.6789 | Higher |
| Cyclomatic (sum) | 26 | **18 ✅** | 41 | +8 | Higher | -15 | Lower | -23 | Lower |
| Cognitive (sum) | 14 | **9 ✅** | 44 | +5 | Higher | -30 | Lower | -35 | Lower |
| Halstead Volume | 4552.4406 | **2578.7309 ✅** | 11397.2605 | +1973.7097 | Higher | -6844.8199 | Lower | -8818.5296 | Lower |
| Halstead Effort | 247538.9577 | **77605.2046 ✅** | 1418440.8804 | +169933.7531 | Higher | -1170901.9227 | Lower | -1340835.6757 | Lower |
| SLOC | 131 | **98 ✅** | 281 | +33 | Higher | -150 | Lower | -183 | Lower |
| PLOC | 111 | **78 ✅** | 257 | +33 | Higher | -146 | Lower | -179 | Lower |
| LLOC | 60 | **33 ✅** | 196 | +27 | Higher | -136 | Lower | -163 | Lower |
| Comments (CLOC) | 2 | **9 ✅** | 0 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | 20 | **16 ✅** | 24 | +4 | Higher | -4 | Lower | -8 | Lower |
| Functions | 6 | 6 | 11 | 0 | Same | -5 | Lower | -5 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |

## p02061

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.9142 | **64.2563 ✅** | 55.0750 | -5.3422 | Lower | +3.8392 | Higher | +9.1813 | Higher |
| MI (SEI) | 10.8216 | **52.8436 ✅** | 5.2829 | -42.0220 | Lower | +5.5388 | Higher | +47.5608 | Higher |
| Cyclomatic (sum) | 15 | **13 ✅** | 15 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 25 | **9 ✅** | 25 | +16 | Higher | 0 | Same | -16 | Lower |
| Halstead Volume | 2209.7490 | **1205.9418 ✅** | 2328.0908 | +1003.8072 | Higher | -118.3417 | Lower | -1122.1489 | Lower |
| Halstead Effort | 107626.1089 | **50524.8041 ✅** | 158237.4187 | +57101.3048 | Higher | -50611.3098 | Lower | -107712.6146 | Lower |
| SLOC | 69 | **62 ✅** | 86 | +7 | Higher | -17 | Lower | -24 | Lower |
| PLOC | 59 | **39 ✅** | 74 | +20 | Higher | -15 | Lower | -35 | Lower |
| LLOC | 47 | **31 ✅** | 62 | +16 | Higher | -15 | Lower | -31 | Lower |
| Comments (CLOC) | 0 | **15 ✅** | 0 | -15 | Lower | 0 | Same | +15 | Higher |
| Blank Lines | **10 ✅** | 12 | 12 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02104

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.2188 | **73.5369 ✅** | 60.8633 | -16.3182 | Lower | -3.6445 | Lower | +12.6736 | Higher |
| MI (SEI) | 34.3731 | **63.0388 ✅** | 29.6458 | -28.6656 | Lower | +4.7273 | Higher | +33.3930 | Higher |
| Cyclomatic (sum) | 13 | **7 ✅** | 13 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 14 | **2 ✅** | 15 | +12 | Higher | -1 | Lower | -13 | Lower |
| Halstead Volume | 3059.8464 | **1567.1339 ✅** | 1905.9763 | +1492.7125 | Higher | +1153.8700 | Higher | -338.8424 | Lower |
| Halstead Effort | 86541.1096 | **18297.5799 ✅** | 111038.4930 | +68243.5298 | Higher | -24497.3834 | Lower | -92740.9132 | Lower |
| SLOC | 71 | **35 ✅** | 66 | +36 | Higher | +5 | Higher | -31 | Lower |
| PLOC | 51 | **22 ✅** | 52 | +29 | Higher | -1 | Lower | -30 | Lower |
| LLOC | 43 | **10 ✅** | 48 | +33 | Higher | -5 | Lower | -38 | Lower |
| Comments (CLOC) | **9 ✅** | 7 | 3 | +2 | Higher | +6 | Higher | +4 | Higher |
| Blank Lines | 12 | **6 ✅** | 12 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02114

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.0367 | 57.6931 | **65.8437 ✅** | +7.3436 | Higher | -0.8070 | Lower | -8.1506 | Lower |
| MI (SEI) | **53.0963 ✅** | 37.8012 | 35.4095 | +15.2950 | Higher | +17.6867 | Higher | +2.3917 | Higher |
| Cyclomatic (sum) | 10 | 20 | 10 | -10 | Lower | 0 | Same | +10 | Higher |
| Cognitive (sum) | 21 | **14 ✅** | 21 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | **1627.3763 ✅** | 3462.3837 | 1864.7978 | -1835.0074 | Lower | -237.4215 | Lower | +1597.5859 | Higher |
| Halstead Effort | **52169.0350 ✅** | 122444.0062 | 77362.4707 | -70274.9712 | Lower | -25193.4357 | Lower | +45081.5355 | Higher |
| SLOC | 56 | 60 | **51 ✅** | -4 | Lower | +5 | Higher | +9 | Higher |
| PLOC | **38 ✅** | 46 | 43 | -8 | Lower | -5 | Lower | +3 | Higher |
| LLOC | 29 | **22 ✅** | 40 | +7 | Higher | -11 | Lower | -18 | Lower |
| Comments (CLOC) | **13 ✅** | 9 | 2 | +4 | Higher | +11 | Higher | +7 | Higher |
| Blank Lines | 11 | **7 ✅** | 8 | +4 | Higher | +3 | Higher | -1 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p02153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.9475 | **39.2807 ✅** | 31.3065 | -1.3332 | Lower | +6.6410 | Higher | +7.9742 | Higher |
| MI (SEI) | -8.3513 | **2.9395 ✅** | -17.9830 | -11.2908 | Lower | +9.6317 | Higher | +20.9225 | Higher |
| Cyclomatic (sum) | 34 | **33 ✅** | 43 | +1 | Higher | -9 | Lower | -10 | Lower |
| Cognitive (sum) | 43 | **24 ✅** | 47 | +19 | Higher | -4 | Lower | -23 | Lower |
| Halstead Volume | **5675.2724 ✅** | 5763.0485 | 6761.2452 | -87.7760 | Lower | -1085.9728 | Lower | -998.1968 | Lower |
| Halstead Effort | 356686.9858 | **206678.7378 ✅** | 605131.4486 | +150008.2479 | Higher | -248444.4629 | Lower | -398452.7108 | Lower |
| SLOC | 142 | **132 ✅** | 178 | +10 | Higher | -36 | Lower | -46 | Lower |
| PLOC | 126 | **111 ✅** | 150 | +15 | Higher | -24 | Lower | -39 | Lower |
| LLOC | 77 | **44 ✅** | 103 | +33 | Higher | -26 | Lower | -59 | Lower |
| Comments (CLOC) | 2 | **8 ✅** | 2 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | **16 ✅** | 19 | 26 | -3 | Lower | -10 | Lower | -7 | Lower |
| Functions | 4 | 4 | 9 | 0 | Same | -5 | Lower | -5 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **4 ✅** | 7 | 6 | -3 | Lower | -2 | Lower | +1 | Higher |

## p02162

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.1817 | **78.0828 ✅** | 70.2945 | -5.9012 | Lower | +1.8872 | Higher | +7.7883 | Higher |
| MI (SEI) | 60.2340 | **69.6989 ✅** | 55.8229 | -9.4649 | Lower | +4.4111 | Higher | +13.8760 | Higher |
| Cyclomatic (sum) | 11 | **8 ✅** | 14 | +3 | Higher | -3 | Lower | -6 | Lower |
| Cognitive (sum) | 13 | **5 ✅** | 13 | +8 | Higher | 0 | Same | -8 | Lower |
| Halstead Volume | 835.2930 | **625.5088 ✅** | 980.4320 | +209.7842 | Higher | -145.1390 | Lower | -354.9232 | Lower |
| Halstead Effort | 18446.0546 | **8913.5007 ✅** | 24131.9971 | +9532.5539 | Higher | -5685.9426 | Lower | -15218.4964 | Lower |
| SLOC | 44 | **35 ✅** | 45 | +9 | Higher | -1 | Lower | -10 | Lower |
| PLOC | 32 | **24 ✅** | 33 | +8 | Higher | -1 | Lower | -9 | Lower |
| LLOC | 16 | **9 ✅** | 14 | +7 | Higher | +2 | Higher | -5 | Lower |
| Comments (CLOC) | **8 ✅** | 7 | 7 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 4 | 0 | Same | -3 | Lower | -3 | Lower |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p02166

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.0030 | 60.6380 | **66.6732 ✅** | +5.3650 | Higher | -0.6702 | Lower | -6.0352 | Lower |
| MI (SEI) | **44.6928 ✅** | 42.5528 | 33.0147 | +2.1400 | Higher | +11.6781 | Higher | +9.5381 | Higher |
| Cyclomatic (sum) | 14 | 14 | **13 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | **10 ✅** | 12 | 11 | -2 | Lower | -1 | Lower | +1 | Higher |
| Halstead Volume | **1716.3884 ✅** | 2433.9687 | 1795.6947 | -717.5803 | Lower | -79.3063 | Lower | +638.2740 | Higher |
| Halstead Effort | 79297.1455 | **51015.0007 ✅** | 81704.1107 | +28282.1448 | Higher | -2406.9652 | Lower | -30689.1100 | Lower |
| SLOC | 49 | 61 | **47 ✅** | -12 | Lower | +2 | Higher | +14 | Higher |
| PLOC | **39 ✅** | 43 | 41 | -4 | Lower | -2 | Lower | +2 | Higher |
| LLOC | 31 | **27 ✅** | 33 | +4 | Higher | -2 | Lower | -6 | Lower |
| Comments (CLOC) | 5 | **10 ✅** | 1 | -5 | Lower | +4 | Higher | +9 | Higher |
| Blank Lines | 6 | 9 | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02186

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.8315 | **53.8048 ✅** | 37.9715 | -14.9733 | Lower | +0.8599 | Higher | +15.8333 | Higher |
| MI (SEI) | -17.2352 | **32.5259 ✅** | -18.3740 | -49.7611 | Lower | +1.1388 | Higher | +50.8999 | Higher |
| Cyclomatic (sum) | 24 | **17 ✅** | 25 | +7 | Higher | -1 | Lower | -8 | Lower |
| Cognitive (sum) | 14 | **10 ✅** | 14 | +4 | Higher | 0 | Same | -4 | Lower |
| Halstead Volume | 6414.2156 | **3998.6629 ✅** | 7090.9281 | +2415.5527 | Higher | -676.7125 | Lower | -3092.2652 | Lower |
| Halstead Effort | 588891.3458 | **132783.8809 ✅** | 688661.3229 | +456107.4648 | Higher | -99769.9771 | Lower | -555877.4420 | Lower |
| SLOC | 149 | **76 ✅** | 150 | +73 | Higher | -1 | Lower | -74 | Lower |
| PLOC | 123 | **66 ✅** | 129 | +57 | Higher | -6 | Lower | -63 | Lower |
| LLOC | 63 | **33 ✅** | 77 | +30 | Higher | -14 | Lower | -44 | Lower |
| Comments (CLOC) | 0 | **12 ✅** | 0 | -12 | Lower | 0 | Same | +12 | Higher |
| Blank Lines | 26 | **9 ✅** | 21 | +17 | Higher | +5 | Higher | -12 | Lower |
| Functions | 9 | **6 ✅** | 9 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 11 | **2 ✅** | 13 | +9 | Higher | -2 | Lower | -11 | Lower |

## p02190

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.6121 ✅** | 72.2210 | 70.6751 | +6.3911 | Higher | +7.9370 | Higher | +1.5459 | Higher |
| MI (SEI) | **71.5831 ✅** | 60.8678 | 58.1577 | +10.7154 | Higher | +13.4254 | Higher | +2.7101 | Higher |
| Cyclomatic (sum) | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 2 | **1 ✅** | 7 | +1 | Higher | -5 | Lower | -6 | Lower |
| Halstead Volume | 2089.7353 | 1768.8393 | **971.7835 ✅** | +320.8960 | Higher | +1117.9518 | Higher | +797.0558 | Higher |
| Halstead Effort | 32693.6156 | **17025.0779 ✅** | 24564.5267 | +15668.5377 | Higher | +8129.0889 | Higher | -7539.4488 | Lower |
| SLOC | **23 ✅** | 36 | 48 | -13 | Lower | -25 | Lower | -12 | Lower |
| PLOC | **21 ✅** | 23 | 31 | -2 | Lower | -10 | Lower | -8 | Lower |
| LLOC | **10 ✅** | 12 | 19 | -2 | Lower | -9 | Lower | -7 | Lower |
| Comments (CLOC) | 5 | 7 | **9 ✅** | -2 | Lower | -4 | Lower | -2 | Lower |
| Blank Lines | **-3 ✅** | 7 | 9 | -10 | Lower | -12 | Lower | -2 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02205

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.0154 | 74.4971 | **80.8524 ✅** | +4.5183 | Higher | -1.8370 | Lower | -6.3553 | Lower |
| MI (SEI) | **74.8829 ✅** | 72.3942 | 68.1516 | +2.4886 | Higher | +6.7313 | Higher | +4.2426 | Higher |
| Cyclomatic (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 749.5321 | 1303.8226 | **708.4702 ✅** | -554.2905 | Lower | +41.0619 | Higher | +595.3524 | Higher |
| Halstead Effort | **13931.5210 ✅** | 16834.1649 | 14523.6394 | -2902.6439 | Lower | -592.1184 | Lower | +2310.5255 | Higher |
| SLOC | 33 | 36 | **30 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| PLOC | 21 | **20 ✅** | 21 | +1 | Higher | 0 | Same | -1 | Lower |
| LLOC | 20 | **10 ✅** | 20 | +10 | Higher | 0 | Same | -10 | Lower |
| Comments (CLOC) | 9 | **13 ✅** | 4 | -4 | Lower | +5 | Higher | +9 | Higher |
| Blank Lines | 6 | 8 | 6 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02242

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.4067 ✅** | 46.9607 | 49.5015 | +15.4460 | Higher | +12.9052 | Higher | -2.5408 | Lower |
| MI (SEI) | **17.4894 ✅** | 13.7284 | 6.0927 | +3.7609 | Higher | +11.3967 | Higher | +7.6358 | Higher |
| Cyclomatic (sum) | 31 | **17 ✅** | 25 | +14 | Higher | +6 | Higher | -8 | Lower |
| Cognitive (sum) | 30 | **18 ✅** | 23 | +12 | Higher | +7 | Higher | -5 | Lower |
| Halstead Volume | 3876.8422 | 4216.2908 | **3002.9158 ✅** | -339.4486 | Lower | +873.9263 | Higher | +1213.3749 | Higher |
| Halstead Effort | **106089.2622 ✅** | 111297.2028 | 173124.6265 | -5207.9406 | Lower | -67035.3643 | Lower | -61827.4238 | Lower |
| SLOC | **37 ✅** | 114 | 97 | -77 | Lower | -60 | Lower | +17 | Higher |
| PLOC | **63 ✅** | 82 | 86 | -19 | Lower | -23 | Lower | -4 | Lower |
| LLOC | 47 | **42 ✅** | 65 | +5 | Higher | -18 | Lower | -23 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 1 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | **-26 ✅** | 24 | 11 | -50 | Lower | -37 | Lower | +13 | Higher |
| Functions | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |

## p02249

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.9967 | **59.8683 ✅** | 43.7296 | -11.8716 | Lower | +4.2671 | Higher | +16.1387 | Higher |
| MI (SEI) | 6.8544 | **31.4155 ✅** | 3.8700 | -24.5611 | Lower | +2.9844 | Higher | +27.5455 | Higher |
| Cyclomatic (sum) | 24 | **14 ✅** | 27 | +10 | Higher | -3 | Lower | -13 | Lower |
| Cognitive (sum) | 38 | **23 ✅** | 45 | +15 | Higher | -7 | Lower | -22 | Lower |
| Halstead Volume | 3812.6044 | **2822.2716 ✅** | 3880.9084 | +990.3328 | Higher | -68.3040 | Lower | -1058.6368 | Lower |
| Halstead Effort | 238526.0607 | **58097.4926 ✅** | 297648.7972 | +180428.5681 | Higher | -59122.7365 | Lower | -239551.3046 | Lower |
| SLOC | 100 | **61 ✅** | 124 | +39 | Higher | -24 | Lower | -63 | Lower |
| PLOC | 84 | **49 ✅** | 108 | +35 | Higher | -24 | Lower | -59 | Lower |
| LLOC | 68 | **23 ✅** | 82 | +45 | Higher | -14 | Lower | -59 | Lower |
| Comments (CLOC) | 2 | 4 | 4 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 14 | **8 ✅** | 14 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

## p02256

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.7655 | **68.2410 ✅** | 62.7796 | -4.4755 | Lower | +0.9859 | Higher | +5.4613 | Higher |
| MI (SEI) | **53.2546 ✅** | 47.3969 | 45.4578 | +5.8578 | Higher | +7.7968 | Higher | +1.9391 | Higher |
| Cyclomatic (sum) | 13 | **11 ✅** | 12 | +2 | Higher | +1 | Higher | -1 | Lower |
| Cognitive (sum) | 13 | **4 ✅** | 15 | +9 | Higher | -2 | Lower | -11 | Lower |
| Halstead Volume | 1394.1864 | **1196.7415 ✅** | 1315.0185 | +197.4449 | Higher | +79.1679 | Higher | -118.2770 | Lower |
| Halstead Effort | 41586.5887 | **32427.8331 ✅** | 42878.9958 | +9158.7556 | Higher | -1292.4071 | Lower | -10451.1627 | Lower |
| SLOC | 61 | **50 ✅** | 67 | +11 | Higher | -6 | Lower | -17 | Lower |
| PLOC | 47 | **38 ✅** | 51 | +9 | Higher | -4 | Lower | -13 | Lower |
| LLOC | 26 | **24 ✅** | 36 | +2 | Higher | -10 | Lower | -12 | Lower |
| Comments (CLOC) | **16 ✅** | 5 | 11 | +11 | Higher | +5 | Higher | -6 | Lower |
| Blank Lines | 10 | **8 ✅** | 12 | +2 | Higher | -2 | Lower | -4 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 4 | 2 | -3 | Lower | -1 | Lower | +2 | Higher |

## p02266

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.6478 | **75.2781 ✅** | 52.7585 | -20.6303 | Lower | +1.8893 | Higher | +22.5196 | Higher |
| MI (SEI) | 16.0901 | **65.9909 ✅** | 13.0738 | -49.9008 | Lower | +3.0163 | Higher | +52.9171 | Higher |
| Cyclomatic (sum) | 16 | **8 ✅** | 16 | +8 | Higher | 0 | Same | -8 | Lower |
| Cognitive (sum) | 41 | **7 ✅** | 29 | +34 | Higher | +12 | Higher | -22 | Lower |
| Halstead Volume | 1959.8510 | **765.7091 ✅** | 2390.0562 | +1194.1420 | Higher | -430.2052 | Lower | -1624.3472 | Lower |
| Halstead Effort | 62306.9311 | **13782.7633 ✅** | 86174.8058 | +48524.1678 | Higher | -23867.8746 | Lower | -72392.0424 | Lower |
| SLOC | 92 | **39 ✅** | 97 | +53 | Higher | -5 | Lower | -58 | Lower |
| PLOC | 83 | **30 ✅** | 86 | +53 | Higher | -3 | Lower | -56 | Lower |
| LLOC | 51 | **19 ✅** | 56 | +32 | Higher | -5 | Lower | -37 | Lower |
| Comments (CLOC) | 2 | **8 ✅** | 2 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | 9 | **4 ✅** | 11 | +5 | Higher | -2 | Lower | -7 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02276

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.2903 | 60.9553 | **68.3465 ✅** | +5.3349 | Higher | -2.0563 | Lower | -7.3912 | Lower |
| MI (SEI) | 39.0702 | 50.9512 | **52.8574 ✅** | -11.8810 | Lower | -13.7872 | Lower | -1.9061 | Lower |
| Cyclomatic (sum) | 11 | 13 | **9 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Cognitive (sum) | 12 | 11 | **10 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Halstead Volume | 1452.3896 | 1311.1011 | **1281.1554 ✅** | +141.2885 | Higher | +171.2343 | Higher | +29.9458 | Higher |
| Halstead Effort | 54815.9951 | 49773.2838 | **44938.9878 ✅** | +5042.7113 | Higher | +9877.0073 | Higher | +4834.2960 | Higher |
| SLOC | 53 | 74 | **50 ✅** | -21 | Lower | +3 | Higher | +24 | Higher |
| PLOC | 45 | 43 | **41 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| LLOC | 28 | **24 ✅** | 26 | +4 | Higher | +2 | Higher | -2 | Lower |
| Comments (CLOC) | 3 | **22 ✅** | 8 | -19 | Lower | -5 | Lower | +14 | Higher |
| Blank Lines | 7 | 13 | 7 | -6 | Lower | 0 | Same | +6 | Higher |
| Functions | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p02277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.1737 | **59.2271 ✅** | 58.3747 | -2.0534 | Lower | -1.2010 | Lower | +0.8524 | Higher |
| MI (SEI) | 29.1391 | **39.6311 ✅** | 10.0433 | -10.4920 | Lower | +19.0958 | Higher | +29.5878 | Higher |
| Cyclomatic (sum) | 15 | **14 ✅** | 15 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 17 | **12 ✅** | 17 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | 2107.7855 | **1904.7520 ✅** | 2146.9269 | +203.0335 | Higher | -39.1414 | Lower | -242.1749 | Lower |
| Halstead Effort | 87587.0323 | **51029.6351 ✅** | 86341.2771 | +36557.3972 | Higher | +1245.7553 | Higher | -35311.6419 | Lower |
| SLOC | 78 | 72 | 72 | +6 | Higher | +6 | Higher | 0 | Same |
| PLOC | 61 | **50 ✅** | 61 | +11 | Higher | 0 | Same | -11 | Lower |
| LLOC | 38 | **25 ✅** | 38 | +13 | Higher | 0 | Same | -13 | Lower |
| Comments (CLOC) | 6 | **11 ✅** | 0 | -5 | Lower | +6 | Higher | +11 | Higher |
| Blank Lines | 11 | 12 | 11 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02279

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.3654 | 53.5205 | **53.9624 ✅** | -0.1551 | Lower | -0.5971 | Lower | -0.4419 | Lower |
| MI (SEI) | 20.6708 | **26.1463 ✅** | 3.7796 | -5.4755 | Lower | +16.8912 | Higher | +22.3667 | Higher |
| Cyclomatic (sum) | **15 ✅** | 16 | 16 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 14 | **7 ✅** | 19 | +7 | Higher | -5 | Lower | -12 | Lower |
| Halstead Volume | **2807.2116 ✅** | 3231.8831 | 2861.1075 | -424.6714 | Lower | -53.8959 | Lower | +370.7755 | Higher |
| Halstead Effort | 106918.1470 | **73017.6765 ✅** | 120946.8185 | +33900.4705 | Higher | -14028.6716 | Lower | -47929.1421 | Lower |
| SLOC | 90 | **84 ✅** | 85 | +6 | Higher | +5 | Higher | -1 | Lower |
| PLOC | 71 | **65 ✅** | 73 | +6 | Higher | -2 | Lower | -8 | Lower |
| LLOC | 42 | **12 ✅** | 54 | +30 | Higher | -12 | Lower | -42 | Lower |
| Comments (CLOC) | 5 | **8 ✅** | 0 | -3 | Lower | +5 | Higher | +8 | Higher |
| Blank Lines | 14 | 17 | **12 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02297

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.9148 | 67.3113 | **78.9477 ✅** | +7.6036 | Higher | -4.0329 | Lower | -11.6365 | Lower |
| MI (SEI) | 62.1976 | 50.5331 | **70.6412 ✅** | +11.6644 | Higher | -8.4436 | Lower | -20.1081 | Lower |
| Cyclomatic (sum) | 6 | 8 | **5 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 3 | 5 | **2 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Halstead Volume | 1056.9386 | 1362.8198 | **977.6705 ✅** | -305.8812 | Lower | +79.2681 | Higher | +385.1494 | Higher |
| Halstead Effort | **24573.8224 ✅** | 41974.8505 | 26885.9378 | -17401.0282 | Lower | -2312.1155 | Lower | +15088.9127 | Higher |
| SLOC | 37 | 53 | **30 ✅** | -16 | Lower | +7 | Higher | +23 | Higher |
| PLOC | 25 | 35 | **21 ✅** | -10 | Lower | +4 | Higher | +14 | Higher |
| LLOC | 18 | 22 | **16 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Comments (CLOC) | 6 | **8 ✅** | 6 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 7 | 12 | **5 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02305

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.7997 | 71.5180 | **73.2317 ✅** | +1.2817 | Higher | -0.4320 | Lower | -1.7138 | Lower |
| MI (SEI) | 51.4286 | **66.1628 ✅** | 57.9287 | -14.7342 | Lower | -6.5001 | Lower | +8.2341 | Higher |
| Cyclomatic (sum) | 10 | **8 ✅** | 10 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 1224.0067 | 1577.9961 | 1224.0067 | -353.9894 | Lower | 0 | Same | +353.9894 | Higher |
| Halstead Effort | 41888.2309 | **23955.1218 ✅** | 41888.2309 | +17933.1090 | Higher | 0 | Same | -17933.1090 | Lower |
| SLOC | 38 | 39 | **37 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| PLOC | 31 | **29 ✅** | 31 | +2 | Higher | 0 | Same | -2 | Lower |
| LLOC | 22 | **11 ✅** | 22 | +11 | Higher | 0 | Same | -11 | Lower |
| Comments (CLOC) | 3 | **12 ✅** | 5 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.4143 | **70.0674 ✅** | 67.9861 | -10.6531 | Lower | -8.5718 | Lower | +2.0813 | Higher |
| MI (SEI) | 30.2638 | **61.0692 ✅** | 34.4880 | -30.8054 | Lower | -4.2242 | Lower | +26.5811 | Higher |
| Cyclomatic (sum) | 12 | **7 ✅** | 10 | +5 | Higher | +2 | Higher | -3 | Lower |
| Cognitive (sum) | 11 | **4 ✅** | 11 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 3042.8726 | 2014.6967 | **1491.8479 ✅** | +1028.1759 | Higher | +1551.0248 | Higher | +522.8489 | Higher |
| Halstead Effort | 88198.5581 | **34249.8442 ✅** | 78042.2914 | +53948.7139 | Higher | +10156.2667 | Higher | -43792.4472 | Lower |
| SLOC | 63 | **40 ✅** | 48 | +23 | Higher | +15 | Higher | -8 | Lower |
| PLOC | 51 | **27 ✅** | 41 | +24 | Higher | +10 | Higher | -14 | Lower |
| LLOC | 34 | **12 ✅** | 33 | +22 | Higher | +1 | Higher | -21 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 1 | -6 | Lower | +3 | Higher | +9 | Higher |
| Blank Lines | 9 | **6 ✅** | 7 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |

## p02328

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.3067 ✅** | 61.9504 | 63.0046 | +1.3563 | Higher | +0.3021 | Higher | -1.0542 | Lower |
| MI (SEI) | 30.7058 | **43.2305 ✅** | 16.3156 | -12.5247 | Lower | +14.3902 | Higher | +26.9149 | Higher |
| Cyclomatic (sum) | 11 | 13 | 11 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 14 | 14 | 14 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **1751.5363 ✅** | 2081.0163 | 1856.3208 | -329.4799 | Lower | -104.7845 | Lower | +224.6955 | Higher |
| Halstead Effort | 92831.4248 | **60566.8913 ✅** | 93367.1346 | +32264.5336 | Higher | -535.7097 | Lower | -32800.2433 | Lower |
| SLOC | 60 | 60 | 60 | 0 | Same | 0 | Same | 0 | Same |
| PLOC | 51 | **45 ✅** | 52 | +6 | Higher | -1 | Lower | -7 | Lower |
| LLOC | 41 | **23 ✅** | 40 | +18 | Higher | +1 | Higher | -17 | Lower |
| Comments (CLOC) | 2 | **9 ✅** | 0 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | 9 | **6 ✅** | 8 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02333

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.1299 | 57.0482 | **58.9742 ✅** | -1.9183 | Lower | -3.8443 | Lower | -1.9260 | Lower |
| MI (SEI) | 17.9696 | **23.6803 ✅** | 20.3151 | -5.7107 | Lower | -2.3454 | Lower | +3.3652 | Higher |
| Cyclomatic (sum) | 19 | 18 | **15 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Cognitive (sum) | 13 | **9 ✅** | 11 | +4 | Higher | +2 | Higher | -2 | Lower |
| Halstead Volume | 2514.4680 | **2136.6995 ✅** | 2394 | +377.7685 | Higher | +120.4680 | Higher | -257.3005 | Lower |
| Halstead Effort | 167631.1969 | **124403.3906 ✅** | 157370.2941 | +43227.8063 | Higher | +10260.9028 | Higher | -32966.9035 | Lower |
| SLOC | 79 | 75 | **67 ✅** | +4 | Higher | +12 | Higher | +8 | Higher |
| PLOC | 62 | 63 | **55 ✅** | -1 | Lower | +7 | Higher | +8 | Higher |
| LLOC | 39 | 39 | **38 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 2 | **3 ✅** | 1 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 15 | 11 | 11 | +4 | Higher | +4 | Higher | 0 | Same |
| Functions | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 6 | 8 | **4 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |

## p02340

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **84.7157 ✅** | 71.3281 | 59.9681 | +13.3876 | Higher | +24.7477 | Higher | +11.3601 | Higher |
| MI (SEI) | **93.6136 ✅** | 58.3922 | 31.1057 | +35.2215 | Higher | +62.5079 | Higher | +27.2864 | Higher |
| Cyclomatic (sum) | 17 | 13 | **11 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Cognitive (sum) | 15 | 6 | 6 | +9 | Higher | +9 | Higher | 0 | Same |
| Halstead Volume | 2566.7592 | **1122.6724 ✅** | 3005.3467 | +1444.0868 | Higher | -438.5874 | Lower | -1882.6743 | Lower |
| Halstead Effort | 65314.8550 | **33447.8952 ✅** | 72961.7354 | +31866.9598 | Higher | -7646.8804 | Lower | -39513.8402 | Lower |
| SLOC | **13 ✅** | 41 | 62 | -28 | Lower | -49 | Lower | -21 | Lower |
| PLOC | 42 | **25 ✅** | 44 | +17 | Higher | -2 | Lower | -19 | Lower |
| LLOC | 24 | **16 ✅** | 22 | +8 | Higher | +2 | Higher | -6 | Lower |
| Comments (CLOC) | 7 | 7 | 4 | 0 | Same | +3 | Higher | +3 | Higher |
| Blank Lines | **-32 ✅** | 9 | 14 | -41 | Lower | -46 | Lower | -5 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 4 | 8 | **3 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |

## p02347

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **41.3934 ✅** | 39.9933 | 40.7398 | +1.4001 | Higher | +0.6535 | Higher | -0.7465 | Lower |
| MI (SEI) | -6.2830 | **6.6527 ✅** | -14.3802 | -12.9358 | Lower | +8.0972 | Higher | +21.0329 | Higher |
| Cyclomatic (sum) | 29 | 32 | **25 ✅** | -3 | Lower | +4 | Higher | +7 | Higher |
| Cognitive (sum) | 33 | **25 ✅** | 33 | +8 | Higher | 0 | Same | -8 | Lower |
| Halstead Volume | **4691.6742 ✅** | 4897.0588 | 5162.3322 | -205.3846 | Lower | -470.6581 | Lower | -265.2735 | Lower |
| Halstead Effort | 314768.6849 | **191440.3287 ✅** | 371444.4142 | +123328.3562 | Higher | -56675.7293 | Lower | -180004.0855 | Lower |
| SLOC | **131 ✅** | 135 | 140 | -4 | Lower | -9 | Lower | -5 | Lower |
| PLOC | 113 | **110 ✅** | 121 | +3 | Higher | -8 | Lower | -11 | Lower |
| LLOC | 50 | **36 ✅** | 81 | +14 | Higher | -31 | Lower | -45 | Lower |
| Comments (CLOC) | 1 | **11 ✅** | 0 | -10 | Lower | +1 | Higher | +11 | Higher |
| Blank Lines | **18 ✅** | 21 | 19 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | **4 ✅** | 7 | 5 | -3 | Lower | -1 | Lower | +2 | Higher |
| Closures | 4 | 1 | **0 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Exits | **3 ✅** | 7 | 4 | -4 | Lower | -1 | Lower | +3 | Higher |

## p02353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.6241 | **50.1733 ✅** | 39.6334 | -6.5492 | Lower | +3.9907 | Higher | +10.5398 | Higher |
| MI (SEI) | 3.2570 | **26.7069 ✅** | -9.6265 | -23.4499 | Lower | +12.8834 | Higher | +36.3334 | Higher |
| Cyclomatic (sum) | 23 | **21 ✅** | 25 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | **13 ✅** | 16 | 14 | -3 | Lower | -1 | Lower | +2 | Higher |
| Halstead Volume | 4610.1070 | **3253.1016 ✅** | 5371.1444 | +1357.0054 | Higher | -761.0374 | Lower | -2118.0428 | Lower |
| Halstead Effort | **254653.5274 ✅** | 320273.1001 | 380211.9183 | -65619.5727 | Lower | -125558.3909 | Lower | -59938.8182 | Lower |
| SLOC | 125 | **96 ✅** | 148 | +29 | Higher | -23 | Lower | -52 | Lower |
| PLOC | 108 | **73 ✅** | 131 | +35 | Higher | -23 | Lower | -58 | Lower |
| LLOC | 63 | **50 ✅** | 75 | +13 | Higher | -12 | Lower | -25 | Lower |
| Comments (CLOC) | 4 | **14 ✅** | 1 | -10 | Lower | +3 | Higher | +13 | Higher |
| Blank Lines | 17 | **13 ✅** | 17 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 7 | **6 ✅** | 9 | +1 | Higher | -2 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | **7 ✅** | 11 | +2 | Higher | -2 | Lower | -4 | Lower |

## p02355

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.1194 | **70.5621 ✅** | 53.2864 | -19.4427 | Lower | -2.1670 | Lower | +17.2757 | Higher |
| MI (SEI) | 27.6570 | **57.4228 ✅** | 17.7034 | -29.7658 | Lower | +9.9536 | Higher | +39.7194 | Higher |
| Cyclomatic (sum) | 22 | **12 ✅** | 23 | +10 | Higher | -1 | Lower | -11 | Lower |
| Cognitive (sum) | 27 | **20 ✅** | 28 | +7 | Higher | -1 | Lower | -8 | Lower |
| Halstead Volume | 2864.3553 | **950.0527 ✅** | 2223.6391 | +1914.3027 | Higher | +640.7163 | Higher | -1273.5864 | Lower |
| Halstead Effort | 123340.8764 | **51725.0891 ✅** | 164472.6142 | +71615.7873 | Higher | -41131.7378 | Lower | -112747.5251 | Lower |
| SLOC | 93 | **46 ✅** | 87 | +47 | Higher | +6 | Higher | -41 | Lower |
| PLOC | 69 | **35 ✅** | 71 | +34 | Higher | -2 | Lower | -36 | Lower |
| LLOC | 57 | **27 ✅** | 61 | +30 | Higher | -4 | Lower | -34 | Lower |
| Comments (CLOC) | **13 ✅** | 8 | 3 | +5 | Higher | +10 | Higher | +5 | Higher |
| Blank Lines | 15 | **6 ✅** | 14 | +9 | Higher | +1 | Higher | -8 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |

## p02369

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.7049 | **57.0571 ✅** | 43.5493 | -10.3522 | Lower | +3.1556 | Higher | +13.5078 | Higher |
| MI (SEI) | 7.3004 | **39.1627 ✅** | 3.5431 | -31.8623 | Lower | +3.7574 | Higher | +35.6196 | Higher |
| Cyclomatic (sum) | 18 | **16 ✅** | 18 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 18 | 18 | 22 | 0 | Same | -4 | Lower | -4 | Lower |
| Halstead Volume | 3344.0724 | **1833.5481 ✅** | 4385.0884 | +1510.5243 | Higher | -1041.0160 | Lower | -2551.5403 | Lower |
| Halstead Effort | 127313.6118 | **59317.7572 ✅** | 233094.8530 | +67995.8545 | Higher | -105781.2413 | Lower | -173777.0958 | Lower |
| SLOC | 123 | **81 ✅** | 137 | +42 | Higher | -14 | Lower | -56 | Lower |
| PLOC | 102 | **61 ✅** | 116 | +41 | Higher | -14 | Lower | -55 | Lower |
| LLOC | 57 | **31 ✅** | 82 | +26 | Higher | -25 | Lower | -51 | Lower |
| Comments (CLOC) | 4 | **15 ✅** | 5 | -11 | Lower | -1 | Lower | +10 | Higher |
| Blank Lines | 20 | **12 ✅** | 19 | +8 | Higher | +1 | Higher | -7 | Lower |
| Functions | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 3 | 6 | 3 | -3 | Lower | 0 | Same | +3 | Higher |

## p02370

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.1678 | 56.2513 | **56.5435 ✅** | -1.0835 | Lower | -1.3757 | Lower | -0.2922 | Lower |
| MI (SEI) | 28.3918 | **42.7203 ✅** | 22.0757 | -14.3285 | Lower | +6.3161 | Higher | +20.6446 | Higher |
| Cyclomatic (sum) | 14 | 27 | 14 | -13 | Lower | 0 | Same | +13 | Higher |
| Cognitive (sum) | **11 ✅** | 34 | 18 | -23 | Lower | -7 | Lower | +16 | Higher |
| Halstead Volume | 3982.8629 | 3026.6405 | **2298.3198 ✅** | +956.2224 | Higher | +1684.5432 | Higher | +728.3208 | Higher |
| Halstead Effort | 88760.9453 | **75666.0132 ✅** | 113363.0700 | +13094.9321 | Higher | -24602.1248 | Lower | -37697.0568 | Lower |
| SLOC | 73 | **62 ✅** | 80 | +11 | Higher | -7 | Lower | -18 | Lower |
| PLOC | 55 | **52 ✅** | 65 | +3 | Higher | -10 | Lower | -13 | Lower |
| LLOC | 37 | **33 ✅** | 56 | +4 | Higher | -19 | Lower | -23 | Lower |
| Comments (CLOC) | 7 | **15 ✅** | 3 | -8 | Lower | +4 | Higher | +12 | Higher |
| Blank Lines | 11 | **-4 ✅** | 13 | +15 | Higher | -2 | Lower | -17 | Lower |
| Functions | **2 ✅** | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |

## p02371

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.9909 | **60.4467 ✅** | 41.3042 | -19.4558 | Lower | -0.3133 | Lower | +19.1426 | Higher |
| MI (SEI) | -14.2216 | **37.3403 ✅** | -13.9733 | -51.5619 | Lower | -0.2483 | Lower | +51.3136 | Higher |
| Cyclomatic (sum) | 23 | **13 ✅** | 21 | +10 | Higher | +2 | Higher | -8 | Lower |
| Cognitive (sum) | 21 | **6 ✅** | 21 | +15 | Higher | 0 | Same | -15 | Lower |
| Halstead Volume | 5620.3607 | **2272.6922 ✅** | 5781.2197 | +3347.6685 | Higher | -160.8590 | Lower | -3508.5275 | Lower |
| Halstead Effort | 539371.3545 | **80839.1326 ✅** | 597830.6762 | +458532.2219 | Higher | -58459.3217 | Lower | -516991.5436 | Lower |
| SLOC | 138 | **64 ✅** | 138 | +74 | Higher | 0 | Same | -74 | Lower |
| PLOC | 119 | **48 ✅** | 121 | +71 | Higher | -2 | Lower | -73 | Lower |
| LLOC | 103 | **27 ✅** | 105 | +76 | Higher | -2 | Lower | -78 | Lower |
| Comments (CLOC) | 0 | **7 ✅** | 0 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 19 | **13 ✅** | 17 | +6 | Higher | +2 | Higher | -4 | Lower |
| Functions | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 3 | 1 | 1 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |

## p02393

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **86.9728 ✅** | 75.0137 | 75.3405 | +11.9591 | Higher | +11.6323 | Higher | -0.3267 | Lower |
| MI (SEI) | **82.6635 ✅** | 66.1708 | 60.7264 | +16.4927 | Higher | +21.9371 | Higher | +5.4444 | Higher |
| Cyclomatic (sum) | **4 ✅** | 5 | 13 | -1 | Lower | -9 | Lower | -8 | Lower |
| Cognitive (sum) | 1 | 1 | 7 | 0 | Same | -6 | Lower | -6 | Lower |
| Halstead Volume | **437.5941 ✅** | 1703.8505 | 1240.0092 | -1266.2564 | Lower | -802.4152 | Lower | +463.8412 | Higher |
| Halstead Effort | **5803.8794 ✅** | 26276.5687 | 56656.6129 | -20472.6893 | Lower | -50852.7335 | Lower | -30380.0443 | Lower |
| SLOC | **24 ✅** | 32 | 31 | -8 | Lower | -7 | Lower | +1 | Higher |
| PLOC | **14 ✅** | 22 | 23 | -8 | Lower | -9 | Lower | -1 | Lower |
| LLOC | 7 | 8 | **5 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Comments (CLOC) | 5 | **7 ✅** | 4 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02396

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **89.1626 ✅** | 82.5073 | 83.6279 | +6.6553 | Higher | +5.5347 | Higher | -1.1206 | Lower |
| MI (SEI) | **81.1769 ✅** | 79.9125 | 76.0038 | +1.2644 | Higher | +5.1731 | Higher | +3.9087 | Higher |
| Cyclomatic (sum) | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 10 | 10 | **2 ✅** | 0 | Same | +8 | Higher | +8 | Higher |
| Halstead Volume | **398.5071 ✅** | 451.3218 | 471.4646 | -52.8146 | Lower | -72.9574 | Lower | -20.1428 | Lower |
| Halstead Effort | **5927.7936 ✅** | 7522.0294 | 7425.5671 | -1594.2358 | Lower | -1497.7734 | Lower | +96.4624 | Higher |
| SLOC | **21 ✅** | 30 | 28 | -9 | Lower | -7 | Lower | +2 | Higher |
| PLOC | **17 ✅** | 22 | 18 | -5 | Lower | -1 | Lower | +4 | Higher |
| LLOC | **11 ✅** | 12 | 12 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 3 | **8 ✅** | 5 | -5 | Lower | -2 | Lower | +3 | Higher |
| Blank Lines | 2 | 2 | 6 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02412

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.8991 | 68.9192 | **75.0755 ✅** | +0.9799 | Higher | -5.1764 | Lower | -6.1563 | Lower |
| MI (SEI) | 49.1548 | 46.8786 | **51.7882 ✅** | +2.2762 | Higher | -2.6334 | Lower | -4.9096 | Lower |
| Cyclomatic (sum) | 17 | 15 | **13 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Cognitive (sum) | 29 | **18 ✅** | 25 | +11 | Higher | +4 | Higher | -7 | Lower |
| Halstead Volume | 926.4271 | 999.4184 | **818.9147 ✅** | -72.9912 | Lower | +107.5124 | Higher | +180.5036 | Higher |
| Halstead Effort | 29433.3618 | 33313.9451 | **22520.1554 ✅** | -3880.5833 | Lower | +6913.2063 | Higher | +10793.7896 | Higher |
| SLOC | 45 | 48 | **36 ✅** | -3 | Lower | +9 | Higher | +12 | Higher |
| PLOC | 34 | 36 | **28 ✅** | -2 | Lower | +6 | Higher | +8 | Higher |
| LLOC | 18 | 18 | 19 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 4 | 4 | 2 | 0 | Same | +2 | Higher | +2 | Higher |
| Blank Lines | 7 | 9 | **6 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02418

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 81.4094 | 81.4094 | **82.9927 ✅** | 0 | Same | -1.5834 | Lower | -1.5834 | Lower |
| MI (SEI) | 75.8296 | 77.6687 | **78.3587 ✅** | -1.8391 | Lower | -2.5291 | Lower | -0.6899 | Lower |
| Cyclomatic (sum) | 5 | 5 | **3 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 4 | 4 | **2 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | 549.8390 | 549.8390 | **490.6561 ✅** | 0 | Same | +59.1829 | Higher | +59.1829 | Higher |
| Halstead Effort | 8705.7835 | 8705.7835 | **6407.3912 ✅** | 0 | Same | +2298.3923 | Higher | +2298.3923 | Higher |
| SLOC | 31 | 31 | **30 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| PLOC | 18 | 18 | **17 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| LLOC | 12 | 12 | 12 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 7 | **8 ✅** | 7 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02430

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.7842 ✅** | 58.0918 | 65.5089 | +14.6924 | Higher | +7.2753 | Higher | -7.4171 | Lower |
| MI (SEI) | **51.9762 ✅** | 39.4970 | 42.6355 | +12.4792 | Higher | +9.3407 | Higher | -3.1385 | Lower |
| Cyclomatic (sum) | 13 | 18 | **11 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| Cognitive (sum) | 9 | **7 ✅** | 8 | +2 | Higher | +1 | Higher | -1 | Lower |
| Halstead Volume | **1168.2422 ✅** | 2603.3992 | 1592.3790 | -1435.1571 | Lower | -424.1368 | Lower | +1011.0202 | Higher |
| Halstead Effort | **35248.6860 ✅** | 56301.5495 | 48639.9396 | -21052.8635 | Lower | -13391.2536 | Lower | +7661.6099 | Higher |
| SLOC | **37 ✅** | 66 | 54 | -29 | Lower | -17 | Lower | +12 | Higher |
| PLOC | **32 ✅** | 47 | 45 | -15 | Lower | -13 | Lower | +2 | Higher |
| LLOC | 19 | **18 ✅** | 25 | +1 | Higher | -6 | Lower | -7 | Lower |
| Comments (CLOC) | 3 | **11 ✅** | 5 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | **4 ✅** | 9 | 8 | -5 | Lower | -4 | Lower | +1 | Higher |
| Functions | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 7 | **1 ✅** | 4 | +6 | Higher | +3 | Higher | -3 | Lower |

## p02433

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 35.5030 | **41.4467 ✅** | 31.0603 | -5.9437 | Lower | +4.4427 | Higher | +10.3863 | Higher |
| MI (SEI) | -11.7157 | **-6.5956 ✅** | -20.1563 | -5.1201 | Lower | +8.4405 | Higher | +13.5607 | Higher |
| Cyclomatic (sum) | 41 | **28 ✅** | 49 | +13 | Higher | -8 | Lower | -21 | Lower |
| Cognitive (sum) | 43 | **30 ✅** | 44 | +13 | Higher | -1 | Lower | -14 | Lower |
| Halstead Volume | 4505.8018 | **3694.0155 ✅** | 5160.7977 | +811.7863 | Higher | -654.9959 | Lower | -1466.7822 | Lower |
| Halstead Effort | 235971.9492 | **159047.8904 ✅** | 325742.5557 | +76924.0588 | Higher | -89770.6065 | Lower | -166694.6653 | Lower |
| SLOC | 161 | **143 ✅** | 181 | +18 | Higher | -20 | Lower | -38 | Lower |
| PLOC | 138 | **131 ✅** | 156 | +7 | Higher | -18 | Lower | -25 | Lower |
| LLOC | 84 | **74 ✅** | 97 | +10 | Higher | -13 | Lower | -23 | Lower |
| Comments (CLOC) | **2 ✅** | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 21 | **12 ✅** | 24 | +9 | Higher | -3 | Lower | -12 | Lower |
| Functions | 9 | **6 ✅** | 10 | +3 | Higher | -1 | Lower | -4 | Lower |
| Closures | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 11 | **1 ✅** | 11 | +10 | Higher | 0 | Same | -10 | Lower |

## p02444

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.2966 | **69.7546 ✅** | 66.9635 | -2.4580 | Lower | +0.3331 | Higher | +2.7911 | Higher |
| MI (SEI) | 54.8895 | 54.3313 | **55.4326 ✅** | +0.5582 | Higher | -0.5432 | Lower | -1.1014 | Lower |
| Cyclomatic (sum) | 8 | **7 ✅** | 9 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 7 | **6 ✅** | 9 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | 1450.2317 | 1482.4260 | **1394.0325 ✅** | -32.1944 | Lower | +56.1992 | Higher | +88.3936 | Higher |
| Halstead Effort | 39273.2087 | **37611.2666 ✅** | 40378.8718 | +1661.9421 | Higher | -1105.6631 | Lower | -2767.6052 | Lower |
| SLOC | 52 | **45 ✅** | 53 | +7 | Higher | -1 | Lower | -8 | Lower |
| PLOC | 36 | **31 ✅** | 36 | +5 | Higher | 0 | Same | -5 | Lower |
| LLOC | 23 | **20 ✅** | 27 | +3 | Higher | -4 | Lower | -7 | Lower |
| Comments (CLOC) | 11 | 7 | **12 ✅** | +4 | Higher | -1 | Lower | -5 | Lower |
| Blank Lines | 9 | **8 ✅** | 10 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02449

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.4812 | **53.6247 ✅** | 50.8483 | -4.1436 | Lower | -1.3671 | Lower | +2.7764 | Higher |
| MI (SEI) | 17.4847 | **32.9117 ✅** | 22.3109 | -15.4270 | Lower | -4.8262 | Lower | +10.6008 | Higher |
| Cyclomatic (sum) | 22 | **18 ✅** | 21 | +4 | Higher | +1 | Higher | -3 | Lower |
| Cognitive (sum) | 51 | **13 ✅** | 51 | +38 | Higher | 0 | Same | -38 | Lower |
| Halstead Volume | 2770.7249 | **2259.6109 ✅** | 2595.8877 | +511.1140 | Higher | +174.8372 | Higher | -336.2767 | Lower |
| Halstead Effort | 215077.5185 | **129009.6613 ✅** | 197979.6981 | +86067.8572 | Higher | +17097.8204 | Higher | -68970.0368 | Lower |
| SLOC | 104 | **91 ✅** | 99 | +13 | Higher | +5 | Higher | -8 | Lower |
| PLOC | 90 | **59 ✅** | 86 | +31 | Higher | +4 | Higher | -27 | Lower |
| LLOC | 77 | **44 ✅** | 77 | +33 | Higher | 0 | Same | -33 | Lower |
| Comments (CLOC) | 7 | **15 ✅** | 9 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | 9 | 21 | **8 ✅** | -12 | Lower | +1 | Higher | +13 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 6 | 0 | -6 | Lower | 0 | Same | +6 | Higher |

## p02450

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.5101 | **74.6012 ✅** | 74.0074 | -3.0911 | Lower | -2.4972 | Lower | +0.5938 | Higher |
| MI (SEI) | 55.9135 | **67.0680 ✅** | 49.7382 | -11.1545 | Lower | +6.1753 | Higher | +17.3299 | Higher |
| Cyclomatic (sum) | 8 | 9 | 8 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 7 | **5 ✅** | 7 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 1254.5655 | **904.7639 ✅** | 1254.5655 | +349.8016 | Higher | 0 | Same | -349.8016 | Lower |
| Halstead Effort | 38934.7906 | **23523.8621 ✅** | 38934.7906 | +15410.9286 | Higher | 0 | Same | -15410.9286 | Lower |
| SLOC | 42 | 38 | **36 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| PLOC | 30 | **28 ✅** | 30 | +2 | Higher | 0 | Same | -2 | Lower |
| LLOC | 19 | **13 ✅** | 19 | +6 | Higher | 0 | Same | -6 | Lower |
| Comments (CLOC) | 6 | **9 ✅** | 2 | -3 | Lower | +4 | Higher | +7 | Higher |
| Blank Lines | 6 | **4 ✅** | 5 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p02457

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.6651 | **50.9359 ✅** | 42.3227 | -7.2708 | Lower | +1.3424 | Higher | +8.6132 | Higher |
| MI (SEI) | -9.5490 | **34.7105 ✅** | -11.1802 | -44.2595 | Lower | +1.6312 | Higher | +45.8907 | Higher |
| Cyclomatic (sum) | 31 | **22 ✅** | 34 | +9 | Higher | -3 | Lower | -12 | Lower |
| Cognitive (sum) | 28 | **14 ✅** | 37 | +14 | Higher | -9 | Lower | -23 | Lower |
| Halstead Volume | 3742.5086 | **2520.5634 ✅** | 4355.8085 | +1221.9452 | Higher | -613.2999 | Lower | -1835.2451 | Lower |
| Halstead Effort | 212346.6825 | **113657.5087 ✅** | 351708.3637 | +98689.1738 | Higher | -139361.6812 | Lower | -238050.8550 | Lower |
| SLOC | 119 | **98 ✅** | 118 | +21 | Higher | +1 | Higher | -20 | Lower |
| PLOC | 106 | **80 ✅** | 102 | +26 | Higher | +4 | Higher | -22 | Lower |
| LLOC | 56 | **21 ✅** | 58 | +35 | Higher | -2 | Lower | -37 | Lower |
| Comments (CLOC) | 0 | **24 ✅** | 0 | -24 | Lower | 0 | Same | +24 | Higher |
| Blank Lines | 13 | **9 ✅** | 16 | +4 | Higher | -3 | Lower | -7 | Lower |
| Functions | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 7 | **3 ✅** | 14 | +4 | Higher | -7 | Lower | -11 | Lower |

## p02542

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 13.6290 | **45.3348 ✅** | 11.8619 | -31.7058 | Lower | +1.7671 | Higher | +33.4729 | Higher |
| MI (SEI) | -46.0968 | **17.8898 ✅** | -52.5803 | -63.9866 | Lower | +6.4835 | Higher | +70.4701 | Higher |
| Cyclomatic (sum) | 56 | **23 ✅** | 59 | +33 | Higher | -3 | Lower | -36 | Lower |
| Cognitive (sum) | 119 | **32 ✅** | 133 | +87 | Higher | -14 | Lower | -101 | Lower |
| Halstead Volume | 16189.7516 | **3578.5251 ✅** | 17517.6205 | +12611.2264 | Higher | -1327.8689 | Lower | -13939.0954 | Lower |
| Halstead Effort | 3710451.2085 | **177852.6983 ✅** | 4266395.1113 | +3532598.5102 | Higher | -555943.9028 | Lower | -4088542.4130 | Lower |
| SLOC | 333 | **122 ✅** | 347 | +211 | Higher | -14 | Lower | -225 | Lower |
| PLOC | 301 | **99 ✅** | 327 | +202 | Higher | -26 | Lower | -228 | Lower |
| LLOC | 316 | **46 ✅** | 348 | +270 | Higher | -32 | Lower | -302 | Lower |
| Comments (CLOC) | 1 | **15 ✅** | 0 | -14 | Lower | +1 | Higher | +15 | Higher |
| Blank Lines | 32 | **18 ✅** | 20 | +14 | Higher | +12 | Higher | -2 | Lower |
| Functions | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |

## p02562

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 14.1336 | **57.2602 ✅** | 11.1101 | -43.1266 | Lower | +3.0235 | Higher | +46.1501 | Higher |
| MI (SEI) | -44.7127 | **43.3067 ✅** | -53.3596 | -88.0194 | Lower | +8.6469 | Higher | +96.6663 | Higher |
| Cyclomatic (sum) | 61 | **16 ✅** | 62 | +45 | Higher | -1 | Lower | -46 | Lower |
| Cognitive (sum) | 111 | **14 ✅** | 119 | +97 | Higher | -8 | Lower | -105 | Lower |
| Halstead Volume | 14572.3379 | **2905.8598 ✅** | 16657.8797 | +11666.4782 | Higher | -2085.5418 | Lower | -13752.0200 | Lower |
| Halstead Effort | 2454528.1729 | **78458.2137 ✅** | 3662443.0791 | +2376069.9592 | Higher | -1207914.9062 | Lower | -3583984.8654 | Lower |
| SLOC | 311 | **69 ✅** | 354 | +242 | Higher | -43 | Lower | -285 | Lower |
| PLOC | 274 | **46 ✅** | 327 | +228 | Higher | -53 | Lower | -281 | Lower |
| LLOC | 277 | **23 ✅** | 339 | +254 | Higher | -62 | Lower | -316 | Lower |
| Comments (CLOC) | 1 | **17 ✅** | 0 | -16 | Lower | +1 | Higher | +17 | Higher |
| Blank Lines | 37 | **11 ✅** | 27 | +26 | Higher | +10 | Higher | -16 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 6 | **3 ✅** | 5 | +3 | Higher | +1 | Higher | -2 | Lower |

## p02584

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.0111 | 76.4302 | **78.5172 ✅** | +0.5809 | Higher | -1.5061 | Lower | -2.0870 | Lower |
| MI (SEI) | 61.3313 | **69.5183 ✅** | 66.3396 | -8.1870 | Lower | -5.0083 | Lower | +3.1787 | Higher |
| Cyclomatic (sum) | 8 | 4 | 4 | +4 | Higher | +4 | Higher | 0 | Same |
| Cognitive (sum) | 8 | **1 ✅** | 4 | +7 | Higher | +4 | Higher | -3 | Lower |
| Halstead Volume | 1940.5408 | 1232.2627 | **824.8989 ✅** | +708.2781 | Higher | +1115.6419 | Higher | +407.3638 | Higher |
| Halstead Effort | 32644.9035 | **15643.3350 ✅** | 19797.5725 | +17001.5685 | Higher | +12847.3310 | Higher | -4154.2375 | Lower |
| SLOC | **26 ✅** | 33 | 33 | -7 | Lower | -7 | Lower | 0 | Same |
| PLOC | 26 | **20 ✅** | 22 | +6 | Higher | +4 | Higher | -2 | Lower |
| LLOC | 17 | **9 ✅** | 19 | +8 | Higher | -2 | Lower | -10 | Lower |
| Comments (CLOC) | 3 | **8 ✅** | 5 | -5 | Lower | -2 | Lower | +3 | Higher |
| Blank Lines | **-3 ✅** | 6 | 6 | -9 | Lower | -9 | Lower | 0 | Same |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02585

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.4963 | **68.0014 ✅** | 55.9495 | -13.5052 | Lower | -1.4532 | Lower | +12.0520 | Higher |
| MI (SEI) | 26.4166 | **61.6631 ✅** | 6.5445 | -35.2465 | Lower | +19.8721 | Higher | +55.1186 | Higher |
| Cyclomatic (sum) | 16 | 26 | **15 ✅** | -10 | Lower | +1 | Higher | +11 | Higher |
| Cognitive (sum) | 24 | 26 | 24 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 2887.7852 | **2592.3370 ✅** | 2776.6830 | +295.4482 | Higher | +111.1022 | Higher | -184.3460 | Lower |
| Halstead Effort | 243624.0614 | **66446.9786 ✅** | 251289.8107 | +177177.0828 | Higher | -7665.7493 | Lower | -184842.8322 | Lower |
| SLOC | 82 | **32 ✅** | 77 | +50 | Higher | +5 | Higher | -45 | Lower |
| PLOC | 62 | **39 ✅** | 62 | +23 | Higher | 0 | Same | -23 | Lower |
| LLOC | 64 | **21 ✅** | 64 | +43 | Higher | 0 | Same | -43 | Lower |
| Comments (CLOC) | 7 | **9 ✅** | 0 | -2 | Lower | +7 | Higher | +9 | Higher |
| Blank Lines | 16 | **-13 ✅** | 15 | +29 | Higher | +1 | Higher | -28 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02606

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 84.6612 | 74.8255 | **90.5053 ✅** | +9.8356 | Higher | -5.8441 | Lower | -15.6797 | Lower |
| MI (SEI) | 79.1249 | 61.7029 | **79.7618 ✅** | +17.4220 | Higher | -0.6369 | Lower | -18.0590 | Lower |
| Cyclomatic (sum) | 2 | 7 | 2 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 745.6809 | 1617.0710 | **593.8777 ✅** | -871.3901 | Lower | +151.8032 | Higher | +1023.1933 | Higher |
| Halstead Effort | 9147.0186 | 32787.5081 | **6326.0884 ✅** | -23640.4895 | Lower | +2820.9302 | Higher | +26461.4197 | Higher |
| SLOC | 24 | 32 | **18 ✅** | -8 | Lower | +6 | Higher | +14 | Higher |
| PLOC | 14 | 24 | **12 ✅** | -10 | Lower | +2 | Higher | +12 | Higher |
| LLOC | 11 | 12 | **9 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Comments (CLOC) | 5 | 5 | 2 | 0 | Same | +3 | Higher | +3 | Higher |
| Blank Lines | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02617

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.9796 | **156.5144 ✅** | 76.8035 | -79.5347 | Lower | +0.1761 | Higher | +79.7108 | Higher |
| MI (SEI) | 64.2231 | **150.2034 ✅** | 60.9480 | -85.9804 | Lower | +3.2750 | Higher | +89.2554 | Higher |
| Cyclomatic (sum) | 5 | **1 ✅** | 5 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 5 | **0 ✅** | 4 | +5 | Higher | +1 | Higher | -4 | Lower |
| Halstead Volume | 1060.7435 | **15.5098 ✅** | 999.8260 | +1045.2338 | Higher | +60.9175 | Higher | -984.3162 | Lower |
| Halstead Effort | 28318.0641 | **15.5098 ✅** | 26828.6642 | +28302.5543 | Higher | +1489.4000 | Higher | -26813.1544 | Lower |
| SLOC | 33 | **1 ✅** | 34 | +32 | Higher | -1 | Lower | -33 | Lower |
| PLOC | 21 | **1 ✅** | 23 | +20 | Higher | -2 | Lower | -22 | Lower |
| LLOC | 18 | **1 ✅** | 19 | +17 | Higher | -1 | Lower | -18 | Lower |
| Comments (CLOC) | **5 ✅** | 0 | 4 | +5 | Higher | +1 | Higher | -4 | Lower |
| Blank Lines | 7 | **0 ✅** | 7 | +7 | Higher | 0 | Same | -7 | Lower |
| Functions | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02618

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.8572 | **56.9987 ✅** | 50.8709 | -8.1415 | Lower | -2.0137 | Lower | +6.1278 | Higher |
| MI (SEI) | 17.5483 | **36.4888 ✅** | 7.6656 | -18.9406 | Lower | +9.8827 | Higher | +28.8232 | Higher |
| Cyclomatic (sum) | 19 | **13 ✅** | 19 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 16 | **9 ✅** | 16 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 3567.2427 | **3192.1365 ✅** | 3548.4041 | +375.1062 | Higher | +18.8386 | Higher | -356.2676 | Lower |
| Halstead Effort | 212899.5312 | **67109.6820 ✅** | 217320.0370 | +145789.8492 | Higher | -4420.5059 | Lower | -150210.3550 | Lower |
| SLOC | 104 | **71 ✅** | 92 | +33 | Higher | +12 | Higher | -21 | Lower |
| PLOC | 79 | **51 ✅** | 73 | +28 | Higher | +6 | Higher | -22 | Lower |
| LLOC | 56 | **19 ✅** | 56 | +37 | Higher | 0 | Same | -37 | Lower |
| Comments (CLOC) | 8 | **11 ✅** | 1 | -3 | Lower | +7 | Higher | +10 | Higher |
| Blank Lines | 18 | **12 ✅** | 19 | +6 | Higher | -1 | Lower | -7 | Lower |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p02639

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 85.5522 | **156.5144 ✅** | 78.6770 | -70.9621 | Lower | +6.8752 | Higher | +77.8374 | Higher |
| MI (SEI) | 84.7830 | **150.2034 ✅** | 65.0309 | -65.4205 | Lower | +19.7521 | Higher | +85.1725 | Higher |
| Cyclomatic (sum) | 5 | **1 ✅** | 8 | +4 | Higher | -3 | Lower | -7 | Lower |
| Cognitive (sum) | 3 | **0 ✅** | 5 | +3 | Higher | -2 | Lower | -5 | Lower |
| Halstead Volume | 484.4792 | **15.5098 ✅** | 814.3479 | +468.9694 | Higher | -329.8687 | Lower | -798.8381 | Lower |
| Halstead Effort | 6903.8281 | **15.5098 ✅** | 14709.1590 | +6888.3183 | Higher | -7805.3310 | Lower | -14693.6493 | Lower |
| SLOC | 25 | **1 ✅** | 31 | +24 | Higher | -6 | Lower | -30 | Lower |
| PLOC | 15 | **1 ✅** | 23 | +14 | Higher | -8 | Lower | -22 | Lower |
| LLOC | 8 | **1 ✅** | 14 | +7 | Higher | -6 | Lower | -13 | Lower |
| Comments (CLOC) | **7 ✅** | 0 | 4 | +7 | Higher | +3 | Higher | -4 | Lower |
| Blank Lines | 4 | **0 ✅** | 4 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p02644

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.6445 ✅** | 54.5365 | 56.3360 | +2.1079 | Higher | +0.3085 | Higher | -1.7995 | Lower |
| MI (SEI) | 31.3280 | **36.3780 ✅** | 7.3057 | -5.0500 | Lower | +24.0223 | Higher | +29.0723 | Higher |
| Cyclomatic (sum) | **12 ✅** | 16 | 17 | -4 | Lower | -5 | Lower | -1 | Lower |
| Cognitive (sum) | 27 | 23 | **21 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Halstead Volume | **2889.3321 ✅** | 2977.1797 | 3320.8634 | -87.8476 | Lower | -431.5314 | Lower | -343.6838 | Lower |
| Halstead Effort | 135592.2263 | **84001.1246 ✅** | 214393.3626 | +51591.1018 | Higher | -78801.1363 | Lower | -130392.2380 | Lower |
| SLOC | 76 | 81 | **69 ✅** | -5 | Lower | +7 | Higher | +12 | Higher |
| PLOC | 58 | 59 | **56 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| LLOC | 43 | **25 ✅** | 53 | +18 | Higher | -10 | Lower | -28 | Lower |
| Comments (CLOC) | 8 | **16 ✅** | 0 | -8 | Lower | +8 | Higher | +16 | Higher |
| Blank Lines | **12 ✅** | 13 | 13 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02647

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.3716 | **71.1617 ✅** | 66.0862 | -7.7901 | Lower | -2.7147 | Lower | +5.0755 | Higher |
| MI (SEI) | 43.6447 | **65.9683 ✅** | 20.7614 | -22.3235 | Lower | +22.8833 | Higher | +45.2068 | Higher |
| Cyclomatic (sum) | 11 | **7 ✅** | 11 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 18 | **7 ✅** | 20 | +11 | Higher | -2 | Lower | -13 | Lower |
| Halstead Volume | 1729.8320 | **1511.4933 ✅** | 1602.8552 | +218.3387 | Higher | +126.9768 | Higher | -91.3618 | Lower |
| Halstead Effort | 77305.5946 | **27445.5364 ✅** | 74414.0348 | +49860.0582 | Higher | +2891.5597 | Higher | -46968.4984 | Lower |
| SLOC | 60 | **41 ✅** | 52 | +19 | Higher | +8 | Higher | -11 | Lower |
| PLOC | 48 | **25 ✅** | 47 | +23 | Higher | +1 | Higher | -22 | Lower |
| LLOC | 42 | **14 ✅** | 43 | +28 | Higher | -1 | Lower | -29 | Lower |
| Comments (CLOC) | 8 | **13 ✅** | 0 | -5 | Lower | +8 | Higher | +13 | Higher |
| Blank Lines | 9 | 7 | **5 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.4493 | **73.4448 ✅** | 24.1196 | -22.9956 | Lower | +26.3297 | Higher | +49.3252 | Higher |
| MI (SEI) | 7.3941 | **64.7995 ✅** | -26.9292 | -57.4054 | Lower | +34.3234 | Higher | +91.7287 | Higher |
| Cyclomatic (sum) | 21 | **10 ✅** | 51 | +11 | Higher | -30 | Lower | -41 | Lower |
| Cognitive (sum) | 17 | **7 ✅** | 43 | +10 | Higher | -26 | Lower | -36 | Lower |
| Halstead Volume | 3905.5625 | **921.5188 ✅** | 8393.6677 | +2984.0437 | Higher | -4488.1052 | Lower | -7472.1489 | Lower |
| Halstead Effort | 370599.7807 | **34056.1300 ✅** | 772217.4328 | +336543.6506 | Higher | -401617.6522 | Lower | -738161.3028 | Lower |
| SLOC | 89 | **40 ✅** | 231 | +49 | Higher | -142 | Lower | -191 | Lower |
| PLOC | 73 | **24 ✅** | 184 | +49 | Higher | -111 | Lower | -160 | Lower |
| LLOC | 57 | **13 ✅** | 95 | +44 | Higher | -38 | Lower | -82 | Lower |
| Comments (CLOC) | 1 | **9 ✅** | 3 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | 15 | **7 ✅** | 44 | +8 | Higher | -29 | Lower | -37 | Lower |
| Functions | 6 | **2 ✅** | 21 | +4 | Higher | -15 | Lower | -19 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 6 | **2 ✅** | 21 | +4 | Higher | -15 | Lower | -19 | Lower |

## p02678

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.8390 | 58.2067 | **60.8637 ✅** | +1.6323 | Higher | -1.0247 | Lower | -2.6570 | Lower |
| MI (SEI) | **27.3014 ✅** | 23.2284 | 22.3283 | +4.0730 | Higher | +4.9731 | Higher | +0.9001 | Higher |
| Cyclomatic (sum) | 10 | 17 | **8 ✅** | -7 | Lower | +2 | Higher | +9 | Higher |
| Cognitive (sum) | 8 | 14 | 8 | -6 | Lower | 0 | Same | +6 | Higher |
| Halstead Volume | **2110.9990 ✅** | 2539.8456 | 2268.7174 | -428.8465 | Lower | -157.7183 | Lower | +271.1282 | Higher |
| Halstead Effort | **63035.4131 ✅** | 87543.6129 | 85602.5793 | -24508.1998 | Lower | -22567.1662 | Lower | +1941.0336 | Higher |
| SLOC | 71 | 67 | 67 | +4 | Higher | +4 | Higher | 0 | Same |
| PLOC | 60 | **55 ✅** | 56 | +5 | Higher | +4 | Higher | -1 | Lower |
| LLOC | 36 | **34 ✅** | 46 | +2 | Higher | -10 | Lower | -12 | Lower |
| Comments (CLOC) | **3 ✅** | 2 | 1 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 11 | 11 | 11 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 3 | 4 | **2 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p02703

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.2111 | **39.8380 ✅** | 35.6670 | -1.6269 | Lower | +2.5441 | Higher | +4.1710 | Higher |
| MI (SEI) | -18.3338 | **-1.5455 ✅** | -15.7213 | -16.7883 | Lower | -2.6125 | Lower | +14.1758 | Higher |
| Cyclomatic (sum) | **22 ✅** | 33 | 26 | -11 | Lower | -4 | Lower | +7 | Higher |
| Cognitive (sum) | 21 | **20 ✅** | 22 | +1 | Higher | -1 | Lower | -2 | Lower |
| Halstead Volume | 6324.1498 | **5177.2777 ✅** | 6775.6145 | +1146.8721 | Higher | -451.4647 | Lower | -1598.3368 | Lower |
| Halstead Effort | 272480.5125 | **174761.8844 ✅** | 320243.6653 | +97718.6281 | Higher | -47763.1529 | Lower | -145481.7810 | Lower |
| SLOC | 160 | **132 ✅** | 173 | +28 | Higher | -13 | Lower | -41 | Lower |
| PLOC | 139 | **110 ✅** | 151 | +29 | Higher | -12 | Lower | -41 | Lower |
| LLOC | 121 | **51 ✅** | 121 | +70 | Higher | 0 | Same | -70 | Lower |
| Comments (CLOC) | 0 | **4 ✅** | 1 | -4 | Lower | -1 | Lower | +3 | Higher |
| Blank Lines | **21 ✅** | 22 | 22 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | **3 ✅** | 5 | 6 | -2 | Lower | -3 | Lower | -1 | Lower |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | **3 ✅** | 5 | 5 | -2 | Lower | -2 | Lower | 0 | Same |

## p02727

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.5596 | **61.9960 ✅** | 54.9878 | -2.4364 | Lower | +4.5718 | Higher | +7.0082 | Higher |
| MI (SEI) | 36.4676 | **44.5186 ✅** | 20.4308 | -8.0510 | Lower | +16.0369 | Higher | +24.0879 | Higher |
| Cyclomatic (sum) | 17 | **12 ✅** | 19 | +5 | Higher | -2 | Lower | -7 | Lower |
| Cognitive (sum) | 16 | **2 ✅** | 23 | +14 | Higher | -7 | Lower | -21 | Lower |
| Halstead Volume | **2258.3879 ✅** | 2993.8675 | 2584.1212 | -735.4796 | Lower | -325.7333 | Lower | +409.7463 | Higher |
| Halstead Effort | 93143.2427 | **62588.0138 ✅** | 129708.5279 | +30555.2289 | Higher | -36565.2852 | Lower | -67120.5141 | Lower |
| SLOC | 64 | **54 ✅** | 79 | +10 | Higher | -15 | Lower | -25 | Lower |
| PLOC | 48 | **35 ✅** | 69 | +13 | Higher | -21 | Lower | -34 | Lower |
| LLOC | 31 | **21 ✅** | 50 | +10 | Higher | -19 | Lower | -29 | Lower |
| Comments (CLOC) | 7 | **9 ✅** | 3 | -2 | Lower | +4 | Higher | +6 | Higher |
| Blank Lines | 11 | 10 | 10 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 4 | 3 | **1 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Exits | 4 | **0 ✅** | 1 | +4 | Higher | +3 | Higher | -1 | Lower |

## p02728

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.9054 | **63.8907 ✅** | 39.0621 | -22.9853 | Lower | +1.8434 | Higher | +24.8286 | Higher |
| MI (SEI) | -14.1413 | **50.6431 ✅** | -16.6989 | -64.7844 | Lower | +2.5576 | Higher | +67.3419 | Higher |
| Cyclomatic (sum) | 25 | **8 ✅** | 26 | +17 | Higher | -1 | Lower | -18 | Lower |
| Cognitive (sum) | 17 | **6 ✅** | 16 | +11 | Higher | +1 | Higher | -10 | Lower |
| Halstead Volume | 6610.9159 | **2482.1344 ✅** | 5857.9949 | +4128.7815 | Higher | +752.9210 | Higher | -3375.8605 | Lower |
| Halstead Effort | 818100.8414 | **47603.7918 ✅** | 686658.8767 | +770497.0496 | Higher | +131441.9647 | Higher | -639055.0849 | Lower |
| SLOC | 128 | **54 ✅** | 147 | +74 | Higher | -19 | Lower | -93 | Lower |
| PLOC | 109 | **34 ✅** | 128 | +75 | Higher | -19 | Lower | -94 | Lower |
| LLOC | 89 | **18 ✅** | 94 | +71 | Higher | -5 | Lower | -76 | Lower |
| Comments (CLOC) | 0 | **12 ✅** | 0 | -12 | Lower | 0 | Same | +12 | Higher |
| Blank Lines | 19 | **10 ✅** | 19 | +9 | Higher | 0 | Same | -9 | Lower |
| Functions | 11 | **3 ✅** | 12 | +8 | Higher | -1 | Lower | -9 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |

## p02731

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 91.7465 | 74.7744 | **92.5774 ✅** | +16.9721 | Higher | -0.8310 | Lower | -17.8030 | Lower |
| MI (SEI) | 94.3804 | 63.5627 | **98.4513 ✅** | +30.8177 | Higher | -4.0709 | Lower | -34.8886 | Lower |
| Cyclomatic (sum) | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 336.8853 | 1949.1212 | 336.8853 | -1612.2358 | Lower | 0 | Same | +1612.2358 | Higher |
| Halstead Effort | 3467.9374 | 24976.3492 | 3467.9374 | -21508.4118 | Lower | 0 | Same | +21508.4118 | Higher |
| SLOC | 20 | 32 | **19 ✅** | -12 | Lower | +1 | Higher | +13 | Higher |
| PLOC | 9 | 17 | 9 | -8 | Lower | 0 | Same | +8 | Higher |
| LLOC | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 6 | 6 | **7 ✅** | 0 | Same | -1 | Lower | -1 | Lower |
| Blank Lines | 5 | 10 | 5 | -5 | Lower | 0 | Same | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02733

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.1553 | **62.4546 ✅** | 49.5731 | -14.2993 | Lower | -1.4178 | Lower | +12.8815 | Higher |
| MI (SEI) | 12.9275 | **46.9576 ✅** | 8.9739 | -34.0301 | Lower | +3.9536 | Higher | +37.9837 | Higher |
| Cyclomatic (sum) | 26 | **13 ✅** | 23 | +13 | Higher | +3 | Higher | -10 | Lower |
| Cognitive (sum) | 70 | **14 ✅** | 63 | +56 | Higher | +7 | Higher | -49 | Lower |
| Halstead Volume | 2823.0310 | **2099.1112 ✅** | 2853.0282 | +723.9198 | Higher | -29.9972 | Lower | -753.9170 | Lower |
| Halstead Effort | 164803.9703 | **57646.8408 ✅** | 217089.5062 | +107157.1295 | Higher | -52285.5359 | Lower | -159442.6653 | Lower |
| SLOC | 106 | **58 ✅** | 101 | +48 | Higher | +5 | Higher | -43 | Lower |
| PLOC | 90 | **43 ✅** | 87 | +47 | Higher | +3 | Higher | -44 | Lower |
| LLOC | 79 | **26 ✅** | 85 | +53 | Higher | -6 | Lower | -59 | Lower |
| Comments (CLOC) | 5 | **11 ✅** | 2 | -6 | Lower | +3 | Higher | +9 | Higher |
| Blank Lines | 14 | **8 ✅** | 13 | +6 | Higher | +1 | Higher | -5 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02734

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.9175 | **68.2225 ✅** | 65.5889 | -0.3050 | Lower | +2.3285 | Higher | +2.6335 | Higher |
| MI (SEI) | 23.1997 | **61.6953 ✅** | 41.1567 | -38.4956 | Lower | -17.9570 | Lower | +20.5386 | Higher |
| Cyclomatic (sum) | **9 ✅** | 10 | 10 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 12 | **3 ✅** | 12 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | **2072.0211 ✅** | 2160.9914 | 2083.0574 | -88.9703 | Lower | -11.0363 | Lower | +77.9341 | Higher |
| Halstead Effort | 142016.3254 | **40626.6390 ✅** | 159484.0785 | +101389.6864 | Higher | -17467.7532 | Lower | -118857.4395 | Lower |
| SLOC | 44 | **42 ✅** | 50 | +2 | Higher | -6 | Lower | -8 | Lower |
| PLOC | 38 | **25 ✅** | 39 | +13 | Higher | -1 | Lower | -14 | Lower |
| LLOC | 37 | **14 ✅** | 41 | +23 | Higher | -4 | Lower | -27 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 4 | -13 | Lower | -4 | Lower | +9 | Higher |
| Blank Lines | **6 ✅** | 10 | 7 | -4 | Lower | -1 | Lower | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02743

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.8944 | 78.5743 | **84.3892 ✅** | -4.6799 | Lower | -10.4948 | Lower | -5.8149 | Lower |
| MI (SEI) | 58.1311 | 65.9013 | **83.0033 ✅** | -7.7701 | Lower | -24.8722 | Lower | -17.1020 | Lower |
| Cyclomatic (sum) | 8 | 6 | **4 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Cognitive (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 2042.8743 | 1246.0003 | **633.3094 ✅** | +796.8740 | Higher | +1409.5649 | Higher | +612.6909 | Higher |
| Halstead Effort | 32465.1374 | 15392.2575 | **8893.8674 ✅** | +17072.8800 | Higher | +23571.2700 | Higher | +6498.3901 | Higher |
| SLOC | 31 | 28 | **25 ✅** | +3 | Higher | +6 | Higher | +3 | Higher |
| PLOC | 22 | 21 | **15 ✅** | +1 | Higher | +7 | Higher | +6 | Higher |
| LLOC | 10 | **7 ✅** | 10 | +3 | Higher | 0 | Same | -3 | Lower |
| Comments (CLOC) | 4 | 4 | **7 ✅** | 0 | Same | -3 | Lower | -3 | Lower |
| Blank Lines | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02761

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.6045 | 61.3294 | **62.2542 ✅** | -0.7248 | Lower | -1.6496 | Lower | -0.9248 | Lower |
| MI (SEI) | 30.6438 | **52.8794 ✅** | 30.6046 | -22.2356 | Lower | +0.0392 | Higher | +22.2748 | Higher |
| Cyclomatic (sum) | 19 | **17 ✅** | 19 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 25 | **18 ✅** | 25 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 2067.4117 | **1271.1981 ✅** | 1974.1396 | +796.2137 | Higher | +93.2721 | Higher | -702.9415 | Lower |
| Halstead Effort | 113966.0704 | **54229.3091 ✅** | 105743.8322 | +59736.7613 | Higher | +8222.2382 | Higher | -51514.5231 | Lower |
| SLOC | 60 | 69 | **55 ✅** | -9 | Lower | +5 | Higher | +14 | Higher |
| PLOC | 50 | **42 ✅** | 49 | +8 | Higher | +1 | Higher | -7 | Lower |
| LLOC | 40 | **31 ✅** | 37 | +9 | Higher | +3 | Higher | -6 | Lower |
| Comments (CLOC) | 3 | **22 ✅** | 2 | -19 | Lower | +1 | Higher | +20 | Higher |
| Blank Lines | 8 | 14 | **6 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 6 | 0 | -6 | Lower | 0 | Same | +6 | Higher |

## p02762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **51.6384 ✅** | 50.6955 | 50.6326 | +0.9429 | Higher | +1.0058 | Higher | +0.0628 | Higher |
| MI (SEI) | 0.4267 | **30.3873 ✅** | 7.2788 | -29.9606 | Lower | -6.8521 | Lower | +23.1085 | Higher |
| Cyclomatic (sum) | **16 ✅** | 17 | 19 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | 12 | **7 ✅** | 15 | +5 | Higher | -3 | Lower | -8 | Lower |
| Halstead Volume | **3495.9222 ✅** | 3628.1856 | 3591.7598 | -132.2633 | Lower | -95.8376 | Lower | +36.4257 | Higher |
| Halstead Effort | 198963.5747 | **112256.0612 ✅** | 246429.7671 | +86707.5135 | Higher | -47466.1924 | Lower | -134173.7059 | Lower |
| SLOC | **92 ✅** | 95 | 93 | -3 | Lower | -1 | Lower | +2 | Higher |
| PLOC | 75 | **72 ✅** | 79 | +3 | Higher | -4 | Lower | -7 | Lower |
| LLOC | 64 | **42 ✅** | 68 | +22 | Higher | -4 | Lower | -26 | Lower |
| Comments (CLOC) | 0 | **18 ✅** | 1 | -18 | Lower | -1 | Lower | +17 | Higher |
| Blank Lines | 17 | 17 | **14 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | **0 ✅** | 1 | 3 | -1 | Lower | -3 | Lower | -2 | Lower |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

## p02783

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 84.2719 | 69.2205 | **88.7261 ✅** | +15.0514 | Higher | -4.4542 | Lower | -19.5056 | Lower |
| MI (SEI) | 80.1545 | 60.0641 | **86.9076 ✅** | +20.0905 | Higher | -6.7530 | Lower | -26.8435 | Lower |
| Cyclomatic (sum) | 4 | 6 | **3 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 573.2581 | 1978.3466 | **494.8981 ✅** | -1405.0886 | Lower | +78.3600 | Higher | +1483.4486 | Higher |
| Halstead Effort | 8299.7798 | 32486.5340 | **6520.2821 ✅** | -24186.7542 | Lower | +1779.4977 | Higher | +25966.2520 | Higher |
| SLOC | 26 | 43 | **21 ✅** | -17 | Lower | +5 | Higher | +22 | Higher |
| PLOC | 16 | 27 | **13 ✅** | -11 | Lower | +3 | Higher | +14 | Higher |
| LLOC | 12 | 14 | **10 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Comments (CLOC) | 6 | **11 ✅** | 5 | -5 | Lower | +1 | Higher | +6 | Higher |
| Blank Lines | 5 | 8 | **4 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02812

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.0613 | **81.5802 ✅** | 75.3702 | -7.5189 | Lower | -1.3089 | Lower | +6.2100 | Higher |
| MI (SEI) | 67.0479 | **84.7657 ✅** | 64.9475 | -17.7178 | Lower | +2.1004 | Higher | +19.8183 | Higher |
| Cyclomatic (sum) | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 779.8431 | **509.0490 ✅** | 926.4271 | +270.7942 | Higher | -146.5840 | Lower | -417.3782 | Lower |
| Halstead Effort | 19974.6186 | **16144.1244 ✅** | 22645.9964 | +3830.4942 | Higher | -2671.3778 | Lower | -6501.8719 | Lower |
| SLOC | 43 | **31 ✅** | 37 | +12 | Higher | +6 | Higher | -6 | Lower |
| PLOC | 27 | **22 ✅** | 25 | +5 | Higher | +2 | Higher | -3 | Lower |
| LLOC | 22 | **16 ✅** | 24 | +6 | Higher | -2 | Lower | -8 | Lower |
| Comments (CLOC) | 11 | **13 ✅** | 7 | -2 | Lower | +4 | Higher | +6 | Higher |
| Blank Lines | 8 | **4 ✅** | 7 | +4 | Higher | +1 | Higher | -3 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p02831

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.7766 | 67.7074 | **86.1493 ✅** | +5.0692 | Higher | -13.3727 | Lower | -18.4419 | Lower |
| MI (SEI) | 55.0117 | 56.5518 | **86.1141 ✅** | -1.5401 | Lower | -31.1023 | Lower | -29.5623 | Lower |
| Cyclomatic (sum) | 7 | 5 | **4 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Cognitive (sum) | **1 ✅** | 2 | 3 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | 1813.8734 | 2096.7011 | **512.6840 ✅** | -282.8277 | Lower | +1301.1894 | Higher | +1584.0171 | Higher |
| Halstead Effort | 35850.6742 | 43768.6348 | **7792.7970 ✅** | -7917.9606 | Lower | +28057.8772 | Higher | +35975.8378 | Higher |
| SLOC | 35 | 47 | **24 ✅** | -12 | Lower | +11 | Higher | +23 | Higher |
| PLOC | 26 | 30 | **14 ✅** | -4 | Lower | +12 | Higher | +16 | Higher |
| LLOC | 15 | 15 | **9 ✅** | 0 | Same | +6 | Higher | +6 | Higher |
| Comments (CLOC) | 4 | **11 ✅** | 7 | -7 | Lower | -3 | Lower | +4 | Higher |
| Blank Lines | 5 | 9 | **4 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p02837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.5720 | **156.5144 ✅** | 61.8758 | -101.9423 | Lower | -7.3037 | Lower | +94.6386 | Higher |
| MI (SEI) | 23.8735 | **150.2034 ✅** | 24.9471 | -126.3299 | Lower | -1.0736 | Lower | +125.2563 | Higher |
| Cyclomatic (sum) | 17 | **1 ✅** | 15 | +16 | Higher | +2 | Higher | -14 | Lower |
| Cognitive (sum) | 26 | **0 ✅** | 26 | +26 | Higher | 0 | Same | -26 | Lower |
| Halstead Volume | 3181.9324 | **15.5098 ✅** | 1835.3846 | +3166.4226 | Higher | +1346.5477 | Higher | -1819.8749 | Lower |
| Halstead Effort | 97311.5242 | **15.5098 ✅** | 90348.2886 | +97296.0144 | Higher | +6963.2356 | Higher | -90332.7789 | Lower |
| SLOC | 78 | **1 ✅** | 61 | +77 | Higher | +17 | Higher | -60 | Lower |
| PLOC | 62 | **1 ✅** | 53 | +61 | Higher | +9 | Higher | -52 | Lower |
| LLOC | 37 | **1 ✅** | 37 | +36 | Higher | 0 | Same | -36 | Lower |
| Comments (CLOC) | **5 ✅** | 0 | 1 | +5 | Higher | +4 | Higher | -1 | Lower |
| Blank Lines | 11 | **0 ✅** | 7 | +11 | Higher | +4 | Higher | -7 | Lower |
| Functions | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p02840

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.1392 | **63.8763 ✅** | 49.7514 | -14.7372 | Lower | -0.6123 | Lower | +14.1249 | Higher |
| MI (SEI) | -2.7716 | **48.3537 ✅** | -1.9902 | -51.1253 | Lower | -0.7815 | Lower | +50.3439 | Higher |
| Cyclomatic (sum) | 20 | **12 ✅** | 19 | +8 | Higher | +1 | Higher | -7 | Lower |
| Cognitive (sum) | 24 | **7 ✅** | 23 | +17 | Higher | +1 | Higher | -16 | Lower |
| Halstead Volume | 3769.1219 | **1969.5416 ✅** | 3731.8830 | +1799.5803 | Higher | +37.2389 | Higher | -1762.3414 | Lower |
| Halstead Effort | 266375.4452 | **42820.1523 ✅** | 286243.3683 | +223555.2928 | Higher | -19867.9232 | Lower | -243423.2160 | Lower |
| SLOC | 99 | **55 ✅** | 97 | +44 | Higher | +2 | Higher | -42 | Lower |
| PLOC | 85 | **40 ✅** | 84 | +45 | Higher | +1 | Higher | -44 | Lower |
| LLOC | 72 | **17 ✅** | 73 | +55 | Higher | -1 | Lower | -56 | Lower |
| Comments (CLOC) | 0 | **10 ✅** | 0 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 14 | **9 ✅** | 13 | +5 | Higher | +1 | Higher | -4 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |

## p02846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.3195 ✅** | 68.0962 | 62.1077 | +2.2234 | Higher | +8.2119 | Higher | +5.9885 | Higher |
| MI (SEI) | 48.9765 | **53.5385 ✅** | 33.6835 | -4.5621 | Lower | +15.2930 | Higher | +19.8550 | Higher |
| Cyclomatic (sum) | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 8 | **3 ✅** | 7 | +5 | Higher | +1 | Higher | -4 | Lower |
| Halstead Volume | **1426.2522 ✅** | 2131.5300 | 2484.5223 | -705.2778 | Lower | -1058.2701 | Lower | -352.9923 | Lower |
| Halstead Effort | 48658.9707 | **33901.4770 ✅** | 172260.2128 | +14757.4937 | Higher | -123601.2421 | Lower | -138358.7358 | Lower |
| SLOC | **44 ✅** | 45 | 62 | -1 | Lower | -18 | Lower | -17 | Lower |
| PLOC | 34 | **31 ✅** | 50 | +3 | Higher | -16 | Lower | -19 | Lower |
| LLOC | 20 | **16 ✅** | 53 | +4 | Higher | -33 | Lower | -37 | Lower |
| Comments (CLOC) | 4 | **8 ✅** | 4 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 6 | 6 | 8 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p02862

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.5646 | **63.4778 ✅** | 55.8731 | -0.9132 | Lower | +6.6915 | Higher | +7.6047 | Higher |
| MI (SEI) | 15.8845 | **51.0802 ✅** | 6.2306 | -35.1957 | Lower | +9.6538 | Higher | +44.8496 | Higher |
| Cyclomatic (sum) | 13 | **11 ✅** | 13 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 1948.5933 | **1386.1568 ✅** | 3963.5826 | +562.4365 | Higher | -2014.9893 | Lower | -2577.4258 | Lower |
| Halstead Effort | 91194.1643 | **69086.0524 ✅** | 118181.3243 | +22108.1119 | Higher | -26987.1600 | Lower | -49095.2719 | Lower |
| SLOC | **59 ✅** | 64 | 71 | -5 | Lower | -12 | Lower | -7 | Lower |
| PLOC | 48 | **42 ✅** | 56 | +6 | Higher | -8 | Lower | -14 | Lower |
| LLOC | 28 | 28 | 37 | 0 | Same | -9 | Lower | -9 | Lower |
| Comments (CLOC) | 0 | **15 ✅** | 0 | -15 | Lower | 0 | Same | +15 | Higher |
| Blank Lines | 11 | 11 | 15 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02884

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.7679 | **60.5193 ✅** | 53.6886 | -8.7514 | Lower | -1.9207 | Lower | +6.8306 | Higher |
| MI (SEI) | 22.8567 | **40.5781 ✅** | 3.8937 | -17.7214 | Lower | +18.9630 | Higher | +36.6844 | Higher |
| Cyclomatic (sum) | 20 | **15 ✅** | 21 | +5 | Higher | -1 | Lower | -6 | Lower |
| Cognitive (sum) | 35 | **26 ✅** | 32 | +9 | Higher | +3 | Higher | -6 | Lower |
| Halstead Volume | 2671.8441 | **2051.4750 ✅** | 2603.5937 | +620.3691 | Higher | +68.2504 | Higher | -552.1187 | Lower |
| Halstead Effort | 173135.4994 | **74927.6813 ✅** | 189940.2960 | +98207.8180 | Higher | -16804.7967 | Lower | -115012.6147 | Lower |
| SLOC | 94 | **64 ✅** | 83 | +30 | Higher | +11 | Higher | -19 | Lower |
| PLOC | 69 | **47 ✅** | 66 | +22 | Higher | +3 | Higher | -19 | Lower |
| LLOC | 63 | **28 ✅** | 61 | +35 | Higher | +2 | Higher | -33 | Lower |
| Comments (CLOC) | 8 | **9 ✅** | 0 | -1 | Lower | +8 | Higher | +9 | Higher |
| Blank Lines | 17 | **10 ✅** | 17 | +7 | Higher | 0 | Same | -7 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p02885

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 87.7539 | 72.3633 | **88.9578 ✅** | +15.3906 | Higher | -1.2039 | Lower | -16.5945 | Lower |
| MI (SEI) | 82.5973 | 54.0977 | **84.8791 ✅** | +28.4996 | Higher | -2.2818 | Lower | -30.7814 | Lower |
| Cyclomatic (sum) | 4 | 7 | **3 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 570.8234 | 1798.9044 | **551.0324 ✅** | -1228.0810 | Lower | +19.7911 | Higher | +1247.8721 | Higher |
| Halstead Effort | 9051.6288 | 29581.9842 | **7013.1395 ✅** | -20530.3553 | Lower | +2038.4893 | Higher | +22568.8447 | Higher |
| SLOC | 21 | 36 | **20 ✅** | -15 | Lower | +1 | Higher | +16 | Higher |
| PLOC | 12 | 25 | **11 ✅** | -13 | Lower | +1 | Higher | +14 | Higher |
| LLOC | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Comments (CLOC) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Blank Lines | 5 | 7 | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02923

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.0683 | **77.1540 ✅** | 54.0699 | -4.0857 | Lower | +18.9983 | Higher | +23.0840 | Higher |
| MI (SEI) | 59.9835 | **72.4783 ✅** | 4.3420 | -12.4948 | Lower | +55.6415 | Higher | +68.1363 | Higher |
| Cyclomatic (sum) | **7 ✅** | 8 | 20 | -1 | Lower | -13 | Lower | -12 | Lower |
| Cognitive (sum) | **6 ✅** | 9 | 15 | -3 | Lower | -9 | Lower | -6 | Lower |
| Halstead Volume | 971.7835 | **628.9629 ✅** | 2184 | +342.8206 | Higher | -1212.2165 | Lower | -1555.0371 | Lower |
| Halstead Effort | 32744.8780 | **19534.8466 ✅** | 108714.6667 | +13210.0314 | Higher | -75969.7887 | Lower | -89179.8201 | Lower |
| SLOC | 42 | **37 ✅** | 87 | +5 | Higher | -45 | Lower | -50 | Lower |
| PLOC | 32 | **27 ✅** | 73 | +5 | Higher | -41 | Lower | -46 | Lower |
| LLOC | 24 | **20 ✅** | 46 | +4 | Higher | -22 | Lower | -26 | Lower |
| Comments (CLOC) | 7 | **10 ✅** | 0 | -3 | Lower | +7 | Higher | +10 | Higher |
| Blank Lines | 7 | **5 ✅** | 14 | +2 | Higher | -7 | Lower | -9 | Lower |
| Functions | **1 ✅** | 2 | 9 | -1 | Lower | -8 | Lower | -7 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | **1 ✅** | 2 | 5 | -1 | Lower | -4 | Lower | -3 | Lower |

## p02945

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.2644 | 76.1333 | **76.9657 ✅** | -1.8688 | Lower | -2.7013 | Lower | -0.8324 | Lower |
| MI (SEI) | 59.7853 | 63.4877 | **67.5093 ✅** | -3.7024 | Lower | -7.7240 | Lower | -4.0216 | Lower |
| Cyclomatic (sum) | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 4 | 3 | 3 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | **772.0879 ✅** | 1314.3709 | 775.9559 | -542.2831 | Lower | -3.8680 | Lower | +538.4151 | Higher |
| Halstead Effort | 17325.6514 | 19557.8393 | **15664.6089 ✅** | -2232.1879 | Lower | +1661.0424 | Higher | +3893.2304 | Higher |
| SLOC | 42 | **32 ✅** | 36 | +10 | Higher | +6 | Higher | -4 | Lower |
| PLOC | 29 | **23 ✅** | 24 | +6 | Higher | +5 | Higher | -1 | Lower |
| LLOC | 22 | **13 ✅** | 20 | +9 | Higher | +2 | Higher | -7 | Lower |
| Comments (CLOC) | 6 | 5 | **7 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | 7 | **4 ✅** | 7 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02951

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.9709 | 79.7838 | **81.9433 ✅** | -0.8129 | Lower | -2.9724 | Lower | -2.1595 | Lower |
| MI (SEI) | 73.3874 | 74.4583 | **79.3683 ✅** | -1.0710 | Lower | -5.9810 | Lower | -4.9100 | Lower |
| Cyclomatic (sum) | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 4 | **2 ✅** | 3 | +2 | Higher | +1 | Higher | -1 | Lower |
| Halstead Volume | 691.9735 | 618.5942 | **610.7609 ✅** | +73.3793 | Higher | +81.2126 | Higher | +7.8333 | Higher |
| Halstead Effort | 13899.6414 | 11809.5253 | **11342.7030 ✅** | +2090.1161 | Higher | +2556.9385 | Higher | +466.8223 | Higher |
| SLOC | 33 | 33 | **29 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| PLOC | 22 | 20 | **18 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| LLOC | 17 | 14 | 14 | +3 | Higher | +3 | Higher | 0 | Same |
| Comments (CLOC) | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Blank Lines | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p02953

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.2858 | 69.0020 | **80.3542 ✅** | +0.2838 | Higher | -11.0685 | Lower | -11.3522 | Lower |
| MI (SEI) | 51.4064 | 61.1634 | **72.7722 ✅** | -9.7570 | Lower | -21.3658 | Lower | -11.6089 | Lower |
| Cyclomatic (sum) | 8 | 10 | **6 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Cognitive (sum) | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 2424.0068 | 2165.5056 | **713.6951 ✅** | +258.5013 | Higher | +1710.3117 | Higher | +1451.8105 | Higher |
| Halstead Effort | 42637.6588 | 42024.3425 | **16950.2589 ✅** | +613.3163 | Higher | +25687.3999 | Higher | +25074.0836 | Higher |
| SLOC | 39 | 40 | **30 ✅** | -1 | Lower | +9 | Higher | +10 | Higher |
| PLOC | 30 | 29 | **21 ✅** | +1 | Higher | +9 | Higher | +8 | Higher |
| LLOC | 16 | **13 ✅** | 16 | +3 | Higher | 0 | Same | -3 | Lower |
| Comments (CLOC) | 5 | **11 ✅** | 6 | -6 | Lower | -1 | Lower | +5 | Higher |
| Blank Lines | 5 | 6 | **4 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p02956

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 32.3337 | **63.4367 ✅** | 32.4088 | -31.1029 | Lower | -0.0750 | Lower | +31.0279 | Higher |
| MI (SEI) | -25.2858 | **45.4857 ✅** | -25.1775 | -70.7715 | Lower | -0.1083 | Lower | +70.6632 | Higher |
| Cyclomatic (sum) | 37 | **11 ✅** | 37 | +26 | Higher | 0 | Same | -26 | Lower |
| Cognitive (sum) | 48 | **4 ✅** | 48 | +44 | Higher | 0 | Same | -44 | Lower |
| Halstead Volume | 6751.8192 | **2667.9595 ✅** | 6770.2919 | +4083.8597 | Higher | -18.4727 | Lower | -4102.3324 | Lower |
| Halstead Effort | 1020552.1508 | **56243.4701 ✅** | 970086.1074 | +964308.6806 | Higher | +50466.0434 | Higher | -913842.6373 | Lower |
| SLOC | 182 | **52 ✅** | 181 | +130 | Higher | +1 | Higher | -129 | Lower |
| PLOC | 157 | **35 ✅** | 156 | +122 | Higher | +1 | Higher | -121 | Lower |
| LLOC | 150 | **18 ✅** | 149 | +132 | Higher | +1 | Higher | -131 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 0 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 25 | **9 ✅** | 25 | +16 | Higher | 0 | Same | -16 | Lower |
| Functions | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |

## p02957

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.8025 | 73.0644 | **87.1440 ✅** | +0.7381 | Higher | -13.3415 | Lower | -14.0796 | Lower |
| MI (SEI) | 59.2207 | 57.8698 | **83.3830 ✅** | +1.3510 | Higher | -24.1623 | Lower | -25.5133 | Lower |
| Cyclomatic (sum) | 8 | 8 | **3 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| Cognitive (sum) | 5 | 5 | **1 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Halstead Volume | 807.3024 | 864.6653 | **505.3170 ✅** | -57.3629 | Lower | +301.9854 | Higher | +359.3483 | Higher |
| Halstead Effort | 18918.9564 | 21582.0463 | **6657.5512 ✅** | -2663.0899 | Lower | +12261.4052 | Higher | +14924.4951 | Higher |
| SLOC | 42 | 43 | **23 ✅** | -1 | Lower | +19 | Higher | +20 | Higher |
| PLOC | 29 | 31 | **13 ✅** | -2 | Lower | +16 | Higher | +18 | Higher |
| LLOC | 15 | 17 | **9 ✅** | -2 | Lower | +6 | Higher | +8 | Higher |
| Comments (CLOC) | 6 | 6 | 5 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | 7 | 6 | **5 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 3 | **1 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p02959

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.8833 | 71.2087 | **72.7070 ✅** | -1.3254 | Lower | -2.8237 | Lower | -1.4984 | Lower |
| MI (SEI) | 56.7145 | 58.0513 | **60.9590 ✅** | -1.3368 | Lower | -4.2445 | Lower | -2.9077 | Lower |
| Cyclomatic (sum) | 9 | **5 ✅** | 9 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 15 | **4 ✅** | 15 | +11 | Higher | 0 | Same | -11 | Lower |
| Halstead Volume | 1419.7513 | 1912.3434 | **1302.3633 ✅** | -492.5922 | Lower | +117.3879 | Higher | +609.9801 | Higher |
| Halstead Effort | 63146.6649 | **33944.0960 ✅** | 65299.0509 | +29202.5689 | Higher | -2152.3859 | Lower | -31354.9548 | Lower |
| SLOC | 44 | 39 | **38 ✅** | +5 | Higher | +6 | Higher | +1 | Higher |
| PLOC | 26 | 30 | **24 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| LLOC | 28 | **19 ✅** | 26 | +9 | Higher | +2 | Higher | -7 | Lower |
| Comments (CLOC) | **8 ✅** | 7 | 7 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 10 | **6 ✅** | 8 | +4 | Higher | +2 | Higher | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02988

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.6602 | 68.2955 | **75.8941 ✅** | +6.3647 | Higher | -1.2340 | Lower | -7.5987 | Lower |
| MI (SEI) | **62.1356 ✅** | 52.2835 | 61.8961 | +9.8521 | Higher | +0.2395 | Higher | -9.6126 | Lower |
| Cyclomatic (sum) | 9 | 15 | **8 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Cognitive (sum) | 7 | **6 ✅** | 11 | +1 | Higher | -4 | Lower | -5 | Lower |
| Halstead Volume | 972.0580 | 1841.2577 | **872.7962 ✅** | -869.1997 | Lower | +99.2618 | Higher | +968.4615 | Higher |
| Halstead Effort | 29753.4274 | 39981.5967 | **28457.7500 ✅** | -10228.1693 | Lower | +1295.6774 | Higher | +11523.8467 | Higher |
| SLOC | 37 | 41 | **36 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| PLOC | 25 | 30 | **24 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| LLOC | 19 | **16 ✅** | 18 | +3 | Higher | +1 | Higher | -2 | Lower |
| Comments (CLOC) | 6 | 6 | 5 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p03003

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.8163 | **67.5187 ✅** | 66.4940 | -0.7024 | Lower | +0.3223 | Higher | +1.0247 | Higher |
| MI (SEI) | 47.2762 | **60.2399 ✅** | 45.3210 | -12.9637 | Lower | +1.9551 | Higher | +14.9189 | Higher |
| Cyclomatic (sum) | 10 | **8 ✅** | 11 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | 16 | **9 ✅** | 16 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | **1645.1100 ✅** | 2035.9230 | 1901.6696 | -390.8130 | Lower | -256.5596 | Lower | +134.2534 | Higher |
| Halstead Effort | 91680.6085 | **47425.0306 ✅** | 115012.9769 | +44255.5779 | Higher | -23332.3684 | Lower | -67587.9463 | Lower |
| SLOC | 50 | **46 ✅** | 48 | +4 | Higher | +2 | Higher | -2 | Lower |
| PLOC | 36 | **31 ✅** | 36 | +5 | Higher | 0 | Same | -5 | Lower |
| LLOC | 31 | **20 ✅** | 32 | +11 | Higher | -1 | Lower | -12 | Lower |
| Comments (CLOC) | 6 | **14 ✅** | 5 | -8 | Lower | +1 | Higher | +9 | Higher |
| Blank Lines | 8 | 7 | 7 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03007

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.9212 | **59.0664 ✅** | 47.5709 | -11.1453 | Lower | +0.3502 | Higher | +11.4955 | Higher |
| MI (SEI) | 3.0017 | **49.2311 ✅** | 2.4611 | -46.2294 | Lower | +0.5406 | Higher | +46.7700 | Higher |
| Cyclomatic (sum) | 20 | **16 ✅** | 20 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 36 | **11 ✅** | 36 | +25 | Higher | 0 | Same | -25 | Lower |
| Halstead Volume | 3966 | **2595.3331 ✅** | 4118.8746 | +1370.6669 | Higher | -152.8746 | Lower | -1523.5415 | Lower |
| Halstead Effort | 479419.4118 | **65020.5804 ✅** | 505420.2325 | +414398.8314 | Higher | -26000.8207 | Lower | -440399.6521 | Lower |
| SLOC | 105 | **64 ✅** | 106 | +41 | Higher | -1 | Lower | -42 | Lower |
| PLOC | 97 | **41 ✅** | 99 | +56 | Higher | -2 | Lower | -58 | Lower |
| LLOC | 78 | **16 ✅** | 79 | +62 | Higher | -1 | Lower | -63 | Lower |
| Comments (CLOC) | 1 | **20 ✅** | 1 | -19 | Lower | 0 | Same | +19 | Higher |
| Blank Lines | 8 | 8 | **7 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 8 | **3 ✅** | 8 | +5 | Higher | 0 | Same | -5 | Lower |

## p03029

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **91.7577 ✅** | 78.9028 | 91.0561 | +12.8549 | Higher | +0.7016 | Higher | -12.1534 | Lower |
| MI (SEI) | **90.2357 ✅** | 68.2990 | 88.4917 | +21.9366 | Higher | +1.7440 | Higher | -20.1927 | Lower |
| Cyclomatic (sum) | 2 | 6 | 2 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 466.7648 | 1890.8157 | **451.3769 ✅** | -1424.0509 | Lower | +15.3878 | Higher | +1439.4388 | Higher |
| Halstead Effort | 5251.1035 | 33930.6009 | **4739.4575 ✅** | -28679.4974 | Lower | +511.6460 | Higher | +29191.1434 | Higher |
| SLOC | **18 ✅** | 24 | 19 | -6 | Lower | -1 | Lower | +5 | Higher |
| PLOC | 10 | 22 | 10 | -12 | Lower | 0 | Same | +12 | Higher |
| LLOC | 7 | 10 | 7 | -3 | Lower | 0 | Same | +3 | Higher |
| Comments (CLOC) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Blank Lines | 4 | **-2 ✅** | 5 | +6 | Higher | -1 | Lower | -7 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03034

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.4561 | 64.9102 | **72.1107 ✅** | +3.5459 | Higher | -3.6546 | Lower | -7.2005 | Lower |
| MI (SEI) | 52.7664 | **54.6577 ✅** | 54.3494 | -1.8914 | Lower | -1.5831 | Lower | +0.3083 | Higher |
| Cyclomatic (sum) | 9 | 9 | **8 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 6 | **3 ✅** | 5 | +3 | Higher | +1 | Higher | -2 | Lower |
| Halstead Volume | 1179.3974 | 2641.9925 | **1038.7213 ✅** | -1462.5951 | Lower | +140.6761 | Higher | +1603.2712 | Higher |
| Halstead Effort | 35381.9214 | 68398.2509 | **29863.2380 ✅** | -33016.3294 | Lower | +5518.6835 | Higher | +38535.0129 | Higher |
| SLOC | 51 | 49 | **43 ✅** | +2 | Higher | +8 | Higher | +6 | Higher |
| PLOC | 33 | **27 ✅** | 29 | +6 | Higher | +4 | Higher | -2 | Lower |
| LLOC | 29 | **16 ✅** | 27 | +13 | Higher | +2 | Higher | -11 | Lower |
| Comments (CLOC) | 8 | **13 ✅** | 5 | -5 | Lower | +3 | Higher | +8 | Higher |
| Blank Lines | 10 | 11 | **9 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03043

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.4777 | 65.7078 | **79.1284 ✅** | +8.7699 | Higher | -4.6507 | Lower | -13.4206 | Lower |
| MI (SEI) | **61.2305 ✅** | 51.9473 | 52.9207 | +9.2832 | Higher | +8.3098 | Higher | -0.9734 | Lower |
| Cyclomatic (sum) | 6 | 9 | **5 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | 6 | **2 ✅** | 4 | +4 | Higher | +2 | Higher | -2 | Lower |
| Halstead Volume | 1057.9888 | 2128.0704 | **944.2801 ✅** | -1070.0815 | Lower | +113.7087 | Higher | +1183.7903 | Higher |
| Halstead Effort | 29976.3501 | 34687.5471 | **26243.1179 ✅** | -4711.1970 | Lower | +3733.2322 | Higher | +8444.4292 | Higher |
| SLOC | 38 | 50 | **30 ✅** | -12 | Lower | +8 | Higher | +20 | Higher |
| PLOC | 26 | 31 | **24 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| LLOC | 23 | **14 ✅** | 23 | +9 | Higher | 0 | Same | -9 | Lower |
| Comments (CLOC) | 6 | **10 ✅** | 1 | -4 | Lower | +5 | Higher | +9 | Higher |
| Blank Lines | 6 | 10 | **5 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03051

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.2890 | **70.4488 ✅** | 60.2124 | -10.1597 | Lower | +0.0766 | Higher | +10.2364 | Higher |
| MI (SEI) | 12.4996 | **58.5834 ✅** | 12.3891 | -46.0838 | Lower | +0.1105 | Higher | +46.1943 | Higher |
| Cyclomatic (sum) | 12 | **7 ✅** | 12 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 13 | **3 ✅** | 13 | +10 | Higher | 0 | Same | -10 | Lower |
| Halstead Volume | 2448.6361 | **1872.2288 ✅** | 2484.9789 | +576.4072 | Higher | -36.3429 | Lower | -612.7501 | Lower |
| Halstead Effort | 182344.3979 | **44018.4134 ✅** | 180273.9271 | +138325.9846 | Higher | +2070.4708 | Higher | -136255.5137 | Lower |
| SLOC | 64 | **40 ✅** | 64 | +24 | Higher | 0 | Same | -24 | Lower |
| PLOC | 54 | **28 ✅** | 54 | +26 | Higher | 0 | Same | -26 | Lower |
| LLOC | 43 | **15 ✅** | 43 | +28 | Higher | 0 | Same | -28 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 0 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 10 | **7 ✅** | 10 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p03053

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.9579 | **62.3283 ✅** | 60.0943 | -0.3703 | Lower | +1.8636 | Higher | +2.2339 | Higher |
| MI (SEI) | 40.2492 | **51.8378 ✅** | 12.3206 | -11.5885 | Lower | +27.9286 | Higher | +39.5171 | Higher |
| Cyclomatic (sum) | 13 | 19 | 13 | -6 | Lower | 0 | Same | +6 | Higher |
| Cognitive (sum) | 16 | 23 | 16 | -7 | Lower | 0 | Same | +7 | Higher |
| Halstead Volume | 2078.0145 | **1741.2623 ✅** | 2554.3490 | +336.7522 | Higher | -476.3344 | Lower | -813.0866 | Lower |
| Halstead Effort | **77818.9794 ✅** | 94260.3348 | 117666.6397 | -16441.3554 | Lower | -39847.6603 | Lower | -23406.3049 | Lower |
| SLOC | 60 | **57 ✅** | 63 | +3 | Higher | -3 | Lower | -6 | Lower |
| PLOC | 43 | **39 ✅** | 54 | +4 | Higher | -11 | Lower | -15 | Lower |
| LLOC | 34 | **25 ✅** | 49 | +9 | Higher | -15 | Lower | -24 | Lower |
| Comments (CLOC) | 7 | **15 ✅** | 0 | -8 | Lower | +7 | Higher | +15 | Higher |
| Blank Lines | 10 | 9 | 9 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p03055

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.9809 | **60.0285 ✅** | 31.3812 | -22.0476 | Lower | +6.5997 | Higher | +28.6473 | Higher |
| MI (SEI) | 2.1011 | **39.3284 ✅** | -26.5581 | -37.2273 | Lower | +28.6593 | Higher | +65.8866 | Higher |
| Cyclomatic (sum) | 29 | **14 ✅** | 38 | +15 | Higher | -9 | Lower | -24 | Lower |
| Cognitive (sum) | 42 | **9 ✅** | 59 | +33 | Higher | -17 | Lower | -50 | Lower |
| Halstead Volume | 5354.4739 | **3036.1339 ✅** | 6158.7938 | +2318.3400 | Higher | -804.3199 | Lower | -3122.6599 | Lower |
| Halstead Effort | 652272.2725 | **65346.5142 ✅** | 835052.1060 | +586925.7583 | Higher | -182779.8334 | Lower | -769705.5918 | Lower |
| SLOC | 155 | **59 ✅** | 196 | +96 | Higher | -41 | Lower | -137 | Lower |
| PLOC | 131 | **41 ✅** | 172 | +90 | Higher | -41 | Lower | -131 | Lower |
| LLOC | 127 | **20 ✅** | 145 | +107 | Higher | -18 | Lower | -125 | Lower |
| Comments (CLOC) | **11 ✅** | 8 | 0 | +3 | Higher | +11 | Higher | +8 | Higher |
| Blank Lines | 14 | **10 ✅** | 24 | +4 | Higher | -10 | Lower | -14 | Lower |
| Functions | 3 | 3 | 6 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |

## p03064

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.3825 | 68.4945 | **69.6920 ✅** | -2.1119 | Lower | -3.3095 | Lower | -1.1975 | Lower |
| MI (SEI) | 48.2345 | **55.8658 ✅** | 47.6115 | -7.6313 | Lower | +0.6230 | Higher | +8.2543 | Higher |
| Cyclomatic (sum) | 10 | 8 | **7 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Cognitive (sum) | 12 | **6 ✅** | 8 | +6 | Higher | +4 | Higher | -2 | Lower |
| Halstead Volume | 1681.2405 | 1807.2094 | **1401.0891 ✅** | -125.9688 | Lower | +280.1515 | Higher | +406.1203 | Higher |
| Halstead Effort | 81268.9986 | **32303.8676 ✅** | 60624.0458 | +48965.1311 | Higher | +20644.9528 | Higher | -28320.1783 | Lower |
| SLOC | 51 | **45 ✅** | 46 | +6 | Higher | +5 | Higher | -1 | Lower |
| PLOC | 39 | **29 ✅** | 36 | +10 | Higher | +3 | Higher | -7 | Lower |
| LLOC | 40 | **16 ✅** | 34 | +24 | Higher | +6 | Higher | -18 | Lower |
| Comments (CLOC) | 7 | **9 ✅** | 4 | -2 | Lower | +3 | Higher | +5 | Higher |
| Blank Lines | 9 | 8 | **6 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03074

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.6773 | **66.8382 ✅** | 57.5533 | -10.1609 | Lower | -0.8760 | Lower | +9.2849 | Higher |
| MI (SEI) | 30.1094 | **59.2599 ✅** | 31.3062 | -29.1505 | Lower | -1.1968 | Lower | +27.9537 | Higher |
| Cyclomatic (sum) | 15 | **10 ✅** | 13 | +5 | Higher | +2 | Higher | -3 | Lower |
| Cognitive (sum) | 11 | **10 ✅** | 15 | +1 | Higher | -4 | Lower | -5 | Lower |
| Halstead Volume | 2414.0316 | **1638.1926 ✅** | 2321.0473 | +775.8389 | Higher | +92.9842 | Higher | -682.8547 | Lower |
| Halstead Effort | 104393.4545 | **40124.6505 ✅** | 98361.4575 | +64268.8039 | Higher | +6031.9970 | Higher | -58236.8069 | Lower |
| SLOC | 77 | **50 ✅** | 76 | +27 | Higher | +1 | Higher | -26 | Lower |
| PLOC | 62 | **35 ✅** | 58 | +27 | Higher | +4 | Higher | -23 | Lower |
| LLOC | 52 | **23 ✅** | 46 | +29 | Higher | +6 | Higher | -23 | Lower |
| Comments (CLOC) | 7 | **15 ✅** | 7 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 13 | **9 ✅** | 11 | +4 | Higher | +2 | Higher | -2 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 4 | 2 | 2 | +2 | Higher | +2 | Higher | 0 | Same |

## p03080

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.6381 | **78.2067 ✅** | 73.4944 | -7.5686 | Lower | -2.8563 | Lower | +4.7123 | Higher |
| MI (SEI) | 53.6126 | **69.9597 ✅** | 62.9546 | -16.3471 | Lower | -9.3421 | Lower | +7.0050 | Higher |
| Cyclomatic (sum) | 11 | **5 ✅** | 10 | +6 | Higher | +1 | Higher | -5 | Lower |
| Cognitive (sum) | 10 | **3 ✅** | 12 | +7 | Higher | -2 | Lower | -9 | Lower |
| Halstead Volume | 915.1977 | **763.3676 ✅** | 845.1914 | +151.8301 | Higher | +70.0063 | Higher | -81.8238 | Lower |
| Halstead Effort | 25907.1350 | **13667.2157 ✅** | 17356.6086 | +12239.9193 | Higher | +8550.5264 | Higher | -3689.3929 | Lower |
| SLOC | 47 | **34 ✅** | 41 | +13 | Higher | +6 | Higher | -7 | Lower |
| PLOC | 34 | **21 ✅** | 31 | +13 | Higher | +3 | Higher | -10 | Lower |
| LLOC | 21 | **14 ✅** | 19 | +7 | Higher | +2 | Higher | -5 | Lower |
| Comments (CLOC) | 6 | 7 | **8 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | 8 | 6 | 6 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |

## p03105

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **83.9531 ✅** | 78.3457 | 73.9413 | +5.6073 | Higher | +10.0118 | Higher | +4.4044 | Higher |
| MI (SEI) | **79.6946 ✅** | 67.8560 | 57.9951 | +11.8385 | Higher | +21.6995 | Higher | +9.8609 | Higher |
| Cyclomatic (sum) | **4 ✅** | 6 | 6 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | 3 | 4 | **2 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Halstead Volume | **609.5060 ✅** | 649.6724 | 2211.7977 | -40.1663 | Lower | -1602.2917 | Lower | -1562.1254 | Lower |
| Halstead Effort | **9325.4425 ✅** | 11863.5823 | 33633.5298 | -2538.1398 | Lower | -24308.0873 | Lower | -21769.9475 | Lower |
| SLOC | **26 ✅** | 35 | 31 | -9 | Lower | -5 | Lower | +4 | Higher |
| PLOC | **15 ✅** | 23 | 22 | -8 | Lower | -7 | Lower | +1 | Higher |
| LLOC | 11 | 14 | **10 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Comments (CLOC) | 6 | 6 | 4 | 0 | Same | +2 | Higher | +2 | Higher |
| Blank Lines | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p03106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.6197 ✅** | 69.0956 | 65.7768 | +6.5240 | Higher | +9.8429 | Higher | +3.3188 | Higher |
| MI (SEI) | **63.9698 ✅** | 50.6663 | 31.4857 | +13.3035 | Higher | +32.4841 | Higher | +19.1806 | Higher |
| Cyclomatic (sum) | **10 ✅** | 11 | 17 | -1 | Lower | -7 | Lower | -6 | Lower |
| Cognitive (sum) | 9 | 9 | 30 | 0 | Same | -21 | Lower | -21 | Lower |
| Halstead Volume | **842.2065 ✅** | 1015.3641 | 1229.4484 | -173.1577 | Lower | -387.2419 | Lower | -214.0842 | Lower |
| Halstead Effort | **26181.6361 ✅** | 34847.2972 | 55448.1216 | -8665.6610 | Lower | -29266.4855 | Lower | -20600.8244 | Lower |
| SLOC | **36 ✅** | 50 | 53 | -14 | Lower | -17 | Lower | -3 | Lower |
| PLOC | **26 ✅** | 37 | 45 | -11 | Lower | -19 | Lower | -8 | Lower |
| LLOC | 18 | 18 | 35 | 0 | Same | -17 | Lower | -17 | Lower |
| Comments (CLOC) | 6 | 6 | 1 | 0 | Same | +5 | Higher | +5 | Higher |
| Blank Lines | 7 | 7 | 8 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p03108

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.3654 | **61.4452 ✅** | 52.5379 | -9.0799 | Lower | -0.1725 | Lower | +8.9073 | Higher |
| MI (SEI) | 23.6553 | **41.0563 ✅** | 15.6530 | -17.4010 | Lower | +8.0023 | Higher | +25.4033 | Higher |
| Cyclomatic (sum) | 15 | **12 ✅** | 15 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 10 | **7 ✅** | 10 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 3402.4481 | **1699.7198 ✅** | 3408 | +1702.7284 | Higher | -5.5519 | Lower | -1708.2802 | Lower |
| Halstead Effort | 216541.5198 | **72238.0896 ✅** | 235510.7368 | +144303.4302 | Higher | -18969.2171 | Lower | -163272.6472 | Lower |
| SLOC | 90 | **67 ✅** | 89 | +23 | Higher | +1 | Higher | -22 | Lower |
| PLOC | 74 | **55 ✅** | 75 | +19 | Higher | -1 | Lower | -20 | Lower |
| LLOC | 54 | **31 ✅** | 58 | +23 | Higher | -4 | Lower | -27 | Lower |
| Comments (CLOC) | 8 | **9 ✅** | 3 | -1 | Lower | +5 | Higher | +6 | Higher |
| Blank Lines | 16 | **9 ✅** | 14 | +7 | Higher | +2 | Higher | -5 | Lower |
| Functions | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **3 ✅** | 6 | +2 | Higher | -1 | Lower | -3 | Lower |

## p03115

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **64.3126 ✅** | 60.2436 | 16.6134 | +4.0690 | Higher | +47.6992 | Higher | +43.6302 | Higher |
| MI (SEI) | 18.6099 | **43.6174 ✅** | -38.2812 | -25.0075 | Lower | +56.8911 | Higher | +81.8986 | Higher |
| Cyclomatic (sum) | 15 | **13 ✅** | 68 | +2 | Higher | -53 | Lower | -55 | Lower |
| Cognitive (sum) | **10 ✅** | 13 | 32 | -3 | Lower | -22 | Lower | -19 | Lower |
| Halstead Volume | **1888.8205 ✅** | 2363.2301 | 9205 | -474.4097 | Lower | -7316.1795 | Lower | -6841.7699 | Lower |
| Halstead Effort | 104298.3045 | **53000.5980 ✅** | 986227.8371 | +51297.7065 | Higher | -881929.5325 | Lower | -933227.2391 | Lower |
| SLOC | **52 ✅** | 64 | 280 | -12 | Lower | -228 | Lower | -216 | Lower |
| PLOC | 44 | **43 ✅** | 240 | +1 | Higher | -196 | Lower | -197 | Lower |
| LLOC | 30 | **19 ✅** | 126 | +11 | Higher | -96 | Lower | -107 | Lower |
| Comments (CLOC) | 0 | **12 ✅** | 2 | -12 | Lower | -2 | Lower | +10 | Higher |
| Blank Lines | **8 ✅** | 9 | 38 | -1 | Lower | -30 | Lower | -29 | Lower |
| Functions | 3 | **2 ✅** | 35 | +1 | Higher | -32 | Lower | -33 | Lower |
| Closures | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 3 | 3 | 37 | 0 | Same | -34 | Lower | -34 | Lower |

## p03123

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.0539 | **73.1416 ✅** | 41.6295 | -31.0876 | Lower | +0.4245 | Higher | +31.5121 | Higher |
| MI (SEI) | -5.8618 | **66.5878 ✅** | -1.8548 | -72.4496 | Lower | -4.0070 | Lower | +68.4426 | Higher |
| Cyclomatic (sum) | 25 | **7 ✅** | 25 | +18 | Higher | 0 | Same | -18 | Lower |
| Cognitive (sum) | 24 | **5 ✅** | 24 | +19 | Higher | 0 | Same | -19 | Lower |
| Halstead Volume | 4388.4927 | **1422.1365 ✅** | 4350.5812 | +2966.3562 | Higher | +37.9115 | Higher | -2928.4447 | Lower |
| Halstead Effort | 275377.9165 | **27040.9099 ✅** | 271148.0676 | +248337.0066 | Higher | +4229.8489 | Higher | -244107.1577 | Lower |
| SLOC | 136 | **37 ✅** | 140 | +99 | Higher | -4 | Lower | -103 | Lower |
| PLOC | 117 | **27 ✅** | 120 | +90 | Higher | -3 | Lower | -93 | Lower |
| LLOC | 82 | **15 ✅** | 91 | +67 | Higher | -9 | Lower | -76 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 3 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | 19 | **6 ✅** | 18 | +13 | Higher | +1 | Higher | -12 | Lower |
| Functions | 9 | **2 ✅** | 9 | +7 | Higher | 0 | Same | -7 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 8 | **1 ✅** | 8 | +7 | Higher | 0 | Same | -7 | Lower |

## p03142

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.3371 | **56.0547 ✅** | 53.3705 | -3.7176 | Lower | -1.0334 | Lower | +2.6842 | Higher |
| MI (SEI) | 22.9385 | **32.6317 ✅** | 3.0275 | -9.6932 | Lower | +19.9110 | Higher | +29.6042 | Higher |
| Cyclomatic (sum) | 16 | **14 ✅** | 17 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | 21 | **10 ✅** | 20 | +11 | Higher | +1 | Higher | -10 | Lower |
| Halstead Volume | 3910.9117 | 3505.8148 | **2852.9540 ✅** | +405.0969 | Higher | +1057.9577 | Higher | +652.8607 | Higher |
| Halstead Effort | 119591.5623 | **108461.1441 ✅** | 158997.3224 | +11130.4181 | Higher | -39405.7601 | Lower | -50536.1782 | Lower |
| SLOC | 85 | **72 ✅** | 87 | +13 | Higher | -2 | Lower | -15 | Lower |
| PLOC | 64 | **50 ✅** | 75 | +14 | Higher | -11 | Lower | -25 | Lower |
| LLOC | 49 | **30 ✅** | 47 | +19 | Higher | +2 | Higher | -17 | Lower |
| Comments (CLOC) | 7 | **9 ✅** | 0 | -2 | Lower | +7 | Higher | +9 | Higher |
| Blank Lines | 14 | 16 | **12 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 1 | 1 | 4 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03145

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 82.0036 | 72.6678 | **87.3398 ✅** | +9.3357 | Higher | -5.3363 | Lower | -14.6720 | Lower |
| MI (SEI) | 77.0334 | 61.3088 | **86.6888 ✅** | +15.7247 | Higher | -9.6553 | Lower | -25.3800 | Lower |
| Cyclomatic (sum) | 4 | 6 | **2 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Cognitive (sum) | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | **567.7809 ✅** | 1773.3201 | 584.2015 | -1205.5392 | Lower | -16.4206 | Lower | +1189.1185 | Higher |
| Halstead Effort | 7551.4858 | 28112.3386 | **5448.3142 ✅** | -20560.8528 | Lower | +2103.1716 | Higher | +22664.0244 | Higher |
| SLOC | 30 | 36 | **22 ✅** | -6 | Lower | +8 | Higher | +14 | Higher |
| PLOC | 17 | 25 | **12 ✅** | -8 | Lower | +5 | Higher | +13 | Higher |
| LLOC | 10 | 11 | **9 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Comments (CLOC) | 7 | 7 | 6 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03157

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.2764 ✅** | 50.4716 | 54.9848 | +5.8048 | Higher | +1.2915 | Higher | -4.5132 | Lower |
| MI (SEI) | 7.4234 | **21.2991 ✅** | 4.9492 | -13.8758 | Lower | +2.4742 | Higher | +16.3500 | Higher |
| Cyclomatic (sum) | 19 | 22 | **13 ✅** | -3 | Lower | +6 | Higher | +9 | Higher |
| Cognitive (sum) | 21 | 35 | **13 ✅** | -14 | Lower | +8 | Higher | +22 | Higher |
| Halstead Volume | **2812.8824 ✅** | 3244.3424 | 4133.1291 | -431.4601 | Lower | -1320.2467 | Lower | -888.7867 | Lower |
| Halstead Effort | 141255.6149 | 157798.1043 | **124195.4892 ✅** | -16542.4894 | Lower | +17060.1257 | Higher | +33602.6150 | Higher |
| SLOC | **71 ✅** | 93 | 74 | -22 | Lower | -3 | Lower | +19 | Higher |
| PLOC | 59 | 71 | 59 | -12 | Lower | 0 | Same | +12 | Higher |
| LLOC | 45 | 53 | **35 ✅** | -8 | Lower | +10 | Higher | +18 | Higher |
| Comments (CLOC) | 0 | **8 ✅** | 0 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | **12 ✅** | 14 | 15 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03164

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.1758 | 61.9416 | **66.9181 ✅** | +4.2342 | Higher | -0.7423 | Lower | -4.9765 | Lower |
| MI (SEI) | 21.0943 | **48.1821 ✅** | 45.3865 | -27.0877 | Lower | -24.2922 | Lower | +2.7955 | Higher |
| Cyclomatic (sum) | 13 | 18 | **10 ✅** | -5 | Lower | +3 | Higher | +8 | Higher |
| Cognitive (sum) | 11 | 13 | **10 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Halstead Volume | 1735.3797 | 2609.6577 | **1613.2244 ✅** | -874.2780 | Lower | +122.1553 | Higher | +996.4333 | Higher |
| Halstead Effort | 57653.1700 | 68873.8039 | **57575.4240 ✅** | -11220.6338 | Lower | +77.7460 | Higher | +11298.3799 | Higher |
| SLOC | **49 ✅** | 52 | 50 | -3 | Lower | -1 | Lower | +2 | Higher |
| PLOC | 38 | 37 | **36 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| LLOC | 31 | **16 ✅** | 30 | +15 | Higher | +1 | Higher | -14 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 5 | -11 | Lower | -5 | Lower | +6 | Higher |
| Blank Lines | 11 | **8 ✅** | 9 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 4 | **1 ✅** | 2 | +3 | Higher | +2 | Higher | -1 | Lower |
| Exits | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03165

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.8538 ✅** | 62.7715 | 59.0681 | +0.0822 | Higher | +3.7857 | Higher | +3.7034 | Higher |
| MI (SEI) | **43.3026 ✅** | 39.7098 | 35.9473 | +3.5928 | Higher | +7.3553 | Higher | +3.7625 | Higher |
| Cyclomatic (sum) | 13 | 13 | 14 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | **16 ✅** | 18 | 18 | -2 | Lower | -2 | Lower | 0 | Same |
| Halstead Volume | 1843.1932 | **1777.0506 ✅** | 2144.0210 | +66.1426 | Higher | -300.8278 | Lower | -366.9704 | Lower |
| Halstead Effort | 89279.6728 | **86702.8723 ✅** | 120195.1191 | +2576.8005 | Higher | -30915.4464 | Lower | -33492.2468 | Lower |
| SLOC | **59 ✅** | 60 | 70 | -1 | Lower | -11 | Lower | -10 | Lower |
| PLOC | **43 ✅** | 46 | 52 | -3 | Lower | -9 | Lower | -6 | Lower |
| LLOC | 37 | **34 ✅** | 45 | +3 | Higher | -8 | Lower | -11 | Lower |
| Comments (CLOC) | 8 | 6 | 8 | +2 | Higher | 0 | Same | -2 | Lower |
| Blank Lines | 9 | 9 | 11 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03173

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.3349 | 65.4730 | **68.4016 ✅** | +1.8618 | Higher | -1.0668 | Lower | -2.9286 | Lower |
| MI (SEI) | 43.9819 | **51.3690 ✅** | 24.0000 | -7.3870 | Lower | +19.9819 | Higher | +27.3689 | Higher |
| Cyclomatic (sum) | 9 | 9 | 10 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | **8 ✅** | 9 | 10 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | 1767.3657 | **1564.0809 ✅** | 1684.0086 | +203.2847 | Higher | +83.3571 | Higher | -119.9277 | Lower |
| Halstead Effort | 94642.4319 | **53446.8796 ✅** | 91622.5427 | +41195.5522 | Higher | +3019.8891 | Higher | -38175.6631 | Lower |
| SLOC | 48 | 56 | **45 ✅** | -8 | Lower | +3 | Higher | +11 | Higher |
| PLOC | **35 ✅** | 36 | 37 | -1 | Lower | -2 | Lower | -1 | Lower |
| LLOC | 27 | 27 | 28 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 4 | **11 ✅** | 0 | -7 | Lower | +4 | Higher | +11 | Higher |
| Blank Lines | 9 | 10 | **8 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p03183

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.2719 | 64.3998 | **64.7010 ✅** | -0.1279 | Lower | -0.4290 | Lower | -0.3011 | Lower |
| MI (SEI) | 41.1872 | **52.9347 ✅** | 42.3529 | -11.7475 | Lower | -1.1657 | Lower | +10.5818 | Higher |
| Cyclomatic (sum) | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | **4 ✅** | 7 | 7 | -3 | Lower | -3 | Lower | 0 | Same |
| Halstead Volume | **1577.8620 ✅** | 2183.6176 | 1700.0838 | -605.7556 | Lower | -122.2218 | Lower | +483.5338 | Higher |
| Halstead Effort | **36882.5248 ✅** | 47973.4179 | 44477.8691 | -11090.8931 | Lower | -7595.3444 | Lower | +3495.5487 | Higher |
| SLOC | 61 | **53 ✅** | 58 | +8 | Higher | +3 | Higher | -5 | Lower |
| PLOC | 45 | **36 ✅** | 42 | +9 | Higher | +3 | Higher | -6 | Lower |
| LLOC | 21 | **12 ✅** | 23 | +9 | Higher | -2 | Lower | -11 | Lower |
| Comments (CLOC) | 6 | **13 ✅** | 6 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 10 | **7 ✅** | 10 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03194

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.3479 | 66.0392 | **73.7382 ✅** | +7.3087 | Higher | -0.3904 | Lower | -7.6990 | Lower |
| MI (SEI) | **58.9617 ✅** | 50.5666 | 32.8192 | +8.3951 | Higher | +26.1425 | Higher | +17.7475 | Higher |
| Cyclomatic (sum) | **9 ✅** | 12 | 21 | -3 | Lower | -12 | Lower | -9 | Lower |
| Cognitive (sum) | **9 ✅** | 12 | 14 | -3 | Lower | -5 | Lower | -2 | Lower |
| Halstead Volume | **908.6608 ✅** | 1044.0873 | 2049.0138 | -135.4265 | Lower | -1140.3530 | Lower | -1004.9265 | Lower |
| Halstead Effort | **30400.6314 ✅** | 37123.1044 | 49859.3366 | -6722.4731 | Lower | -19458.7052 | Lower | -12736.2321 | Lower |
| SLOC | 41 | 59 | **26 ✅** | -18 | Lower | +15 | Higher | +33 | Higher |
| PLOC | **28 ✅** | 41 | 30 | -13 | Lower | -2 | Lower | +11 | Higher |
| LLOC | 25 | 27 | **20 ✅** | -2 | Lower | +5 | Higher | +7 | Higher |
| Comments (CLOC) | 6 | **10 ✅** | 0 | -4 | Lower | +6 | Higher | +10 | Higher |
| Blank Lines | 7 | 10 | **-4 ✅** | -3 | Lower | +11 | Higher | +14 | Higher |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 4 | **0 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |

## p03206

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **88.3945 ✅** | 86.6549 | 85.9321 | +1.7396 | Higher | +2.4623 | Higher | +0.7228 | Higher |
| MI (SEI) | **86.9382 ✅** | 85.7940 | 74.4472 | +1.1442 | Higher | +12.4910 | Higher | +11.3468 | Higher |
| Cyclomatic (sum) | 8 | 9 | **6 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Cognitive (sum) | **3 ✅** | 5 | 4 | -2 | Lower | -1 | Lower | +1 | Higher |
| Halstead Volume | **422.8322 ✅** | 425.7542 | 430.8598 | -2.9221 | Lower | -8.0276 | Lower | -5.1055 | Lower |
| Halstead Effort | 4737.6424 | **4405.6309 ✅** | 4452.2177 | +332.0114 | Higher | +285.4247 | Higher | -46.5867 | Lower |
| SLOC | **21 ✅** | 23 | 25 | -2 | Lower | -4 | Lower | -2 | Lower |
| PLOC | **14 ✅** | 16 | 18 | -2 | Lower | -4 | Lower | -2 | Lower |
| LLOC | **4 ✅** | 6 | 11 | -2 | Lower | -7 | Lower | -5 | Lower |
| Comments (CLOC) | 5 | **6 ✅** | 3 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03209

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 81.2809 | 76.9749 | **81.5508 ✅** | +4.3060 | Higher | -0.2700 | Lower | -4.5760 | Lower |
| MI (SEI) | 63.0021 | **73.9475 ✅** | 73.4815 | -10.9454 | Lower | -10.4795 | Lower | +0.4660 | Higher |
| Cyclomatic (sum) | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 6 | **1 ✅** | 6 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | 932.6887 | 1168.5336 | **787.2783 ✅** | -235.8448 | Lower | +145.4104 | Higher | +381.2552 | Higher |
| Halstead Effort | 24465.1430 | **15775.2030 ✅** | 20469.2369 | +8689.9400 | Higher | +3995.9061 | Higher | -4694.0339 | Lower |
| SLOC | **26 ✅** | 32 | 27 | -6 | Lower | -1 | Lower | +5 | Higher |
| PLOC | 22 | **19 ✅** | 20 | +3 | Higher | +2 | Higher | -1 | Lower |
| LLOC | 13 | **7 ✅** | 12 | +6 | Higher | +1 | Higher | -5 | Lower |
| Comments (CLOC) | 2 | **10 ✅** | 5 | -8 | Lower | -3 | Lower | +5 | Higher |
| Blank Lines | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |

## p03213

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **91.2812 ✅** | 67.1664 | 72.7383 | +24.1149 | Higher | +18.5430 | Higher | -5.5719 | Lower |
| MI (SEI) | **82.1031 ✅** | 55.7107 | 30.0528 | +26.3925 | Higher | +52.0503 | Higher | +25.6578 | Higher |
| Cyclomatic (sum) | 14 | 10 | **8 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Cognitive (sum) | 13 | 13 | **9 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Halstead Volume | 2607.3869 | 1637.9200 | **1353.1137 ✅** | +969.4669 | Higher | +1254.2732 | Higher | +284.8063 | Higher |
| Halstead Effort | 73332.7569 | **39144.0752 ✅** | 62919.7869 | +34188.6817 | Higher | +10412.9700 | Higher | -23775.7117 | Lower |
| SLOC | **9 ✅** | 49 | 38 | -40 | Lower | -29 | Lower | +11 | Higher |
| PLOC | 41 | 35 | **32 ✅** | +6 | Higher | +9 | Higher | +3 | Higher |
| LLOC | 24 | 22 | 22 | +2 | Higher | +2 | Higher | 0 | Same |
| Comments (CLOC) | 1 | **11 ✅** | 0 | -10 | Lower | +1 | Higher | +11 | Higher |
| Blank Lines | **-33 ✅** | 7 | 6 | -40 | Lower | -39 | Lower | +1 | Higher |
| Functions | 3 | 2 | 2 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |

## p03216

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.6315 | **62.2773 ✅** | 60.4963 | -4.6458 | Lower | -2.8648 | Lower | +1.7810 | Higher |
| MI (SEI) | 35.5091 | **48.6492 ✅** | 13.0023 | -13.1400 | Lower | +22.5068 | Higher | +35.6468 | Higher |
| Cyclomatic (sum) | 17 | 19 | **14 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Cognitive (sum) | 15 | **11 ✅** | 30 | +4 | Higher | -15 | Lower | -19 | Lower |
| Halstead Volume | 1766.7505 | **1353.1936 ✅** | 1867.3021 | +413.5569 | Higher | -100.5516 | Lower | -514.1085 | Lower |
| Halstead Effort | 65590.6109 | **47277.2017 ✅** | 78185.7447 | +18313.4092 | Higher | -12595.1337 | Lower | -30908.5429 | Lower |
| SLOC | 78 | **62 ✅** | 67 | +16 | Higher | +11 | Higher | -5 | Lower |
| PLOC | 58 | **45 ✅** | 58 | +13 | Higher | 0 | Same | -13 | Lower |
| LLOC | 45 | **29 ✅** | 55 | +16 | Higher | -10 | Lower | -26 | Lower |
| Comments (CLOC) | 10 | **13 ✅** | 0 | -3 | Lower | +10 | Higher | +13 | Higher |
| Blank Lines | 10 | 9 | 9 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03218

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.3688 ✅** | 60.5846 | 71.5482 | +13.7842 | Higher | +2.8207 | Higher | -10.9635 | Lower |
| MI (SEI) | **58.2388 ✅** | 43.1204 | 52.9216 | +15.1184 | Higher | +5.3172 | Higher | -9.8012 | Lower |
| Cyclomatic (sum) | **6 ✅** | 14 | 7 | -8 | Lower | -1 | Lower | +7 | Higher |
| Cognitive (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **920.8124 ✅** | 2877.4272 | 1049.9730 | -1956.6148 | Lower | -129.1605 | Lower | +1827.4543 | Higher |
| Halstead Effort | **20027.6700 ✅** | 55150.6884 | 25724.3373 | -35123.0184 | Lower | -5696.6673 | Lower | +29426.3512 | Higher |
| SLOC | **40 ✅** | 58 | 45 | -18 | Lower | -5 | Lower | +13 | Higher |
| PLOC | **27 ✅** | 42 | 31 | -15 | Lower | -4 | Lower | +11 | Higher |
| LLOC | **15 ✅** | 22 | 16 | -7 | Lower | -1 | Lower | +6 | Higher |
| Comments (CLOC) | 5 | **10 ✅** | 5 | -5 | Lower | 0 | Same | +5 | Higher |
| Blank Lines | **8 ✅** | 9 | 10 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 7 | 2 | -6 | Lower | -1 | Lower | +5 | Higher |

## p03268

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.9929 | 66.9143 | **82.7523 ✅** | +12.0785 | Higher | -3.7594 | Lower | -15.8380 | Lower |
| MI (SEI) | 67.4095 | 54.8380 | **74.9094 ✅** | +12.5716 | Higher | -7.4999 | Lower | -20.0714 | Lower |
| Cyclomatic (sum) | 4 | 5 | **3 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 828.5343 | 2144.8059 | **713.5293 ✅** | -1316.2716 | Lower | +115.0050 | Higher | +1431.2766 | Higher |
| Halstead Effort | 15742.1516 | 38258.6991 | **14066.7205 ✅** | -22516.5476 | Lower | +1675.4311 | Higher | +24191.9787 | Higher |
| SLOC | 32 | 49 | **27 ✅** | -17 | Lower | +5 | Higher | +22 | Higher |
| PLOC | 22 | 27 | **18 ✅** | -5 | Lower | +4 | Higher | +9 | Higher |
| LLOC | 14 | 14 | **11 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Comments (CLOC) | 5 | **11 ✅** | 5 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | 5 | 11 | **4 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.1509 | **69.3805 ✅** | 55.3174 | -13.2297 | Lower | +0.8335 | Higher | +14.0631 | Higher |
| MI (SEI) | 25.8325 | **63.3986 ✅** | 22.6097 | -37.5661 | Lower | +3.2228 | Higher | +40.7889 | Higher |
| Cyclomatic (sum) | 15 | **7 ✅** | 16 | +8 | Higher | -1 | Lower | -9 | Lower |
| Cognitive (sum) | 12 | **4 ✅** | 13 | +8 | Higher | -1 | Lower | -9 | Lower |
| Halstead Volume | 2466.1128 | **2128.9474 ✅** | 2562.0403 | +337.1654 | Higher | -95.9275 | Lower | -433.0929 | Lower |
| Halstead Effort | 133370.0463 | **50266.8137 ✅** | 138350.1763 | +83103.2326 | Higher | -4980.1299 | Lower | -88083.3625 | Lower |
| SLOC | 79 | **41 ✅** | 81 | +38 | Higher | -2 | Lower | -40 | Lower |
| PLOC | 65 | **30 ✅** | 71 | +35 | Higher | -6 | Lower | -41 | Lower |
| LLOC | 51 | **17 ✅** | 55 | +34 | Higher | -4 | Lower | -38 | Lower |
| Comments (CLOC) | 5 | **13 ✅** | 4 | -8 | Lower | +1 | Higher | +9 | Higher |
| Blank Lines | 14 | **8 ✅** | 10 | +6 | Higher | +4 | Higher | -2 | Lower |
| Functions | 6 | **2 ✅** | 6 | +4 | Higher | 0 | Same | -4 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p03278

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.9708 | **61.0893 ✅** | 45.1069 | -15.1186 | Lower | +0.8639 | Higher | +15.9825 | Higher |
| MI (SEI) | -7.4445 | **43.0003 ✅** | -8.5890 | -50.4448 | Lower | +1.1445 | Higher | +51.5893 | Higher |
| Cyclomatic (sum) | 19 | **12 ✅** | 20 | +7 | Higher | -1 | Lower | -8 | Lower |
| Cognitive (sum) | 19 | **5 ✅** | 21 | +14 | Higher | -2 | Lower | -16 | Lower |
| Halstead Volume | 6404.3822 | **2438.0245 ✅** | 6241.4044 | +3966.3577 | Higher | +162.9777 | Higher | -3803.3799 | Lower |
| Halstead Effort | 961204.0383 | **54855.5512 ✅** | 790400.8300 | +906348.4870 | Higher | +170803.2083 | Higher | -735545.2787 | Lower |
| SLOC | 103 | **61 ✅** | 108 | +42 | Higher | -5 | Lower | -47 | Lower |
| PLOC | 91 | **45 ✅** | 96 | +46 | Higher | -5 | Lower | -51 | Lower |
| LLOC | 85 | **17 ✅** | 90 | +68 | Higher | -5 | Lower | -73 | Lower |
| Comments (CLOC) | 0 | **10 ✅** | 0 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 12 | **11 ✅** | 12 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p03294

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.1635 | 68.6718 | **86.0192 ✅** | +3.4916 | Higher | -13.8557 | Lower | -17.3474 | Lower |
| MI (SEI) | 55.6619 | 51.4308 | **81.1859 ✅** | +4.2310 | Higher | -25.5241 | Lower | -29.7551 | Lower |
| Cyclomatic (sum) | 6 | 7 | **3 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Cognitive (sum) | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 1651.0431 | 2103.3682 | **549.4460 ✅** | -452.3251 | Lower | +1101.5971 | Higher | +1553.9222 | Higher |
| Halstead Effort | 27344.2412 | 34541.5508 | **7157.2569 ✅** | -7197.3096 | Lower | +20186.9842 | Higher | +27384.2939 | Higher |
| SLOC | 38 | 43 | **24 ✅** | -5 | Lower | +14 | Higher | +19 | Higher |
| PLOC | 25 | 25 | **15 ✅** | 0 | Same | +10 | Higher | +10 | Higher |
| LLOC | 14 | 14 | **12 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Comments (CLOC) | 5 | **6 ✅** | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | 8 | 12 | **5 ✅** | -4 | Lower | +3 | Higher | +7 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p03296

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.9079 | 77.6062 | **81.0966 ✅** | -3.6983 | Lower | -7.1887 | Lower | -3.4904 | Lower |
| MI (SEI) | 62.1491 | 64.8598 | **72.2501 ✅** | -2.7107 | Lower | -10.1010 | Lower | -7.3903 | Lower |
| Cyclomatic (sum) | 7 | 6 | **5 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Cognitive (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 1041.6346 | 819.7476 | **801.8106 ✅** | +221.8870 | Higher | +239.8241 | Higher | +17.9371 | Higher |
| Halstead Effort | 29632.7095 | **14981.5947 ✅** | 18942.7743 | +14651.1148 | Higher | +10689.9352 | Higher | -3961.1795 | Lower |
| SLOC | 39 | 34 | **28 ✅** | +5 | Higher | +11 | Higher | +6 | Higher |
| PLOC | 31 | 24 | **23 ✅** | +7 | Higher | +8 | Higher | +1 | Higher |
| LLOC | 24 | **18 ✅** | 21 | +6 | Higher | +3 | Higher | -3 | Lower |
| Comments (CLOC) | **7 ✅** | 5 | 5 | +2 | Higher | +2 | Higher | 0 | Same |
| Blank Lines | 6 | 5 | **4 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03311

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.2507 | **65.6500 ✅** | 60.4302 | -2.3992 | Lower | +2.8205 | Higher | +5.2198 | Higher |
| MI (SEI) | 35.9899 | **51.0915 ✅** | 33.0952 | -15.1016 | Lower | +2.8947 | Higher | +17.9963 | Higher |
| Cyclomatic (sum) | 11 | **4 ✅** | 12 | +7 | Higher | -1 | Lower | -8 | Lower |
| Cognitive (sum) | 17 | **2 ✅** | 17 | +15 | Higher | 0 | Same | -15 | Lower |
| Halstead Volume | **1681.6297 ✅** | 2523.8607 | 1972.9015 | -842.2310 | Lower | -291.2717 | Lower | +550.9593 | Higher |
| Halstead Effort | 101704.9653 | **35697.4862 ✅** | 123306.3412 | +66007.4791 | Higher | -21601.3759 | Lower | -87608.8550 | Lower |
| SLOC | 61 | **51 ✅** | 68 | +10 | Higher | -7 | Lower | -17 | Lower |
| PLOC | 48 | **28 ✅** | 55 | +20 | Higher | -7 | Lower | -27 | Lower |
| LLOC | 45 | **15 ✅** | 53 | +30 | Higher | -8 | Lower | -38 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 5 | -6 | Lower | -1 | Lower | +5 | Higher |
| Blank Lines | 9 | 15 | 9 | -6 | Lower | 0 | Same | +6 | Higher |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.1042 | **87.6580 ✅** | 80.0827 | -9.5539 | Lower | -1.9786 | Lower | +7.5753 | Higher |
| MI (SEI) | 66.1274 | **85.3667 ✅** | 67.5499 | -19.2392 | Lower | -1.4224 | Lower | +17.8168 | Higher |
| Cyclomatic (sum) | 4 | **3 ✅** | 5 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 982.9534 | **607.7417 ✅** | 873.5091 | +375.2117 | Higher | +109.4443 | Higher | -265.7674 | Lower |
| Halstead Effort | 21788.8004 | **9139.4995 ✅** | 20964.2174 | +12649.3009 | Higher | +824.5830 | Higher | -11824.7179 | Lower |
| SLOC | 32 | **21 ✅** | 29 | +11 | Higher | +3 | Higher | -8 | Lower |
| PLOC | 21 | **13 ✅** | 20 | +8 | Higher | +1 | Higher | -7 | Lower |
| LLOC | 17 | **8 ✅** | 15 | +9 | Higher | +2 | Higher | -7 | Lower |
| Comments (CLOC) | 5 | 5 | 4 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | 6 | **3 ✅** | 5 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03318

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.7728 | 63.2485 | **75.2868 ✅** | +2.5243 | Higher | -9.5140 | Lower | -12.0382 | Lower |
| MI (SEI) | 31.6310 | **53.0478 ✅** | 33.6277 | -21.4169 | Lower | -1.9967 | Lower | +19.4201 | Higher |
| Cyclomatic (sum) | 11 | 9 | **7 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Cognitive (sum) | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 2494.3295 | 2270.3773 | **1025.2978 ✅** | +223.9522 | Higher | +1469.0317 | Higher | +1245.0796 | Higher |
| Halstead Effort | 59164.5629 | 45407.5466 | **36733.2640 ✅** | +13757.0163 | Higher | +22431.2989 | Higher | +8674.2826 | Higher |
| SLOC | 46 | 57 | **36 ✅** | -11 | Lower | +10 | Higher | +21 | Higher |
| PLOC | 39 | 35 | **32 ✅** | +4 | Higher | +7 | Higher | +3 | Higher |
| LLOC | 26 | **19 ✅** | 28 | +7 | Higher | -2 | Lower | -9 | Lower |
| Comments (CLOC) | 1 | **16 ✅** | 0 | -15 | Lower | +1 | Higher | +16 | Higher |
| Blank Lines | 7 | 10 | **4 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03361

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.6488 | 63.6548 | **70.7857 ✅** | +5.9940 | Higher | -1.1369 | Lower | -7.1309 | Lower |
| MI (SEI) | **51.9563 ✅** | 41.8491 | 27.8468 | +10.1073 | Higher | +24.1096 | Higher | +14.0023 | Higher |
| Cyclomatic (sum) | 14 | 16 | 14 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 32 | 31 | **29 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Halstead Volume | **1483.5314 ✅** | 1540.6490 | 1510.6165 | -57.1176 | Lower | -27.0851 | Lower | +30.0325 | Higher |
| Halstead Effort | 56476.5056 | **47850.7455 ✅** | 52116.2701 | +8625.7602 | Higher | +4360.2355 | Higher | -4265.5247 | Lower |
| SLOC | 41 | 57 | **38 ✅** | -16 | Lower | +3 | Higher | +19 | Higher |
| PLOC | **29 ✅** | 46 | 31 | -17 | Lower | -2 | Lower | +15 | Higher |
| LLOC | **21 ✅** | 32 | 23 | -11 | Lower | -2 | Lower | +9 | Higher |
| Comments (CLOC) | 5 | **6 ✅** | 0 | -1 | Lower | +5 | Higher | +6 | Higher |
| Blank Lines | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p03365

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.2636 | 54.8433 | **62.5721 ✅** | +6.4204 | Higher | -1.3085 | Lower | -7.7288 | Lower |
| MI (SEI) | 14.1094 | **17.4048 ✅** | 15.8953 | -3.2955 | Lower | -1.7859 | Lower | +1.5096 | Higher |
| Cyclomatic (sum) | 14 | 16 | **13 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 2394.2357 | 3286.2988 | **2289.2982 ✅** | -892.0631 | Lower | +104.9375 | Higher | +997.0006 | Higher |
| Halstead Effort | 170840.3037 | **120668.7857 ✅** | 145471.4348 | +50171.5180 | Higher | +25368.8689 | Higher | -24802.6492 | Lower |
| SLOC | 59 | 77 | **56 ✅** | -18 | Lower | +3 | Higher | +21 | Higher |
| PLOC | 48 | 60 | **46 ✅** | -12 | Lower | +2 | Higher | +14 | Higher |
| LLOC | **37 ✅** | 42 | 38 | -5 | Lower | -1 | Lower | +4 | Higher |
| Comments (CLOC) | 0 | **2 ✅** | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 11 | 17 | **10 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Functions | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 6 | **3 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p03369

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 88.0210 | **88.0872 ✅** | 85.4080 | -0.0662 | Lower | +2.6130 | Higher | +2.6793 | Higher |
| MI (SEI) | 89.4111 | **89.5067 ✅** | 79.3391 | -0.0956 | Lower | +10.0720 | Higher | +10.1676 | Higher |
| Cyclomatic (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | 408.4241 | **403.2542 ✅** | 460.7495 | +5.1699 | Higher | -52.3254 | Lower | -57.4953 | Lower |
| Halstead Effort | 4933.3329 | **4870.8857 ✅** | 6582.1353 | +62.4473 | Higher | -1648.8024 | Lower | -1711.2497 | Lower |
| SLOC | 23 | 23 | 26 | 0 | Same | -3 | Lower | -3 | Lower |
| PLOC | 11 | 11 | 14 | 0 | Same | -3 | Lower | -3 | Lower |
| LLOC | 8 | 8 | 10 | 0 | Same | -2 | Lower | -2 | Lower |
| Comments (CLOC) | 7 | 7 | 5 | 0 | Same | +2 | Higher | +2 | Higher |
| Blank Lines | 5 | 5 | 7 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03406

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.8371 | **58.4614 ✅** | 49.0517 | -8.6243 | Lower | +0.7853 | Higher | +9.4096 | Higher |
| MI (SEI) | 11.6896 | **41.8644 ✅** | -2.8978 | -30.1748 | Lower | +14.5874 | Higher | +44.7622 | Higher |
| Cyclomatic (sum) | 20 | **11 ✅** | 20 | +9 | Higher | 0 | Same | -9 | Lower |
| Cognitive (sum) | 32 | **9 ✅** | 32 | +23 | Higher | 0 | Same | -23 | Lower |
| Halstead Volume | 3512.0792 | **1887.4802 ✅** | 3601.4830 | +1624.5989 | Higher | -89.4038 | Lower | -1714.0027 | Lower |
| Halstead Effort | 319414.3581 | **129858.6399 ✅** | 292234.6167 | +189555.7182 | Higher | +27179.7414 | Higher | -162375.9768 | Lower |
| SLOC | 97 | **79 ✅** | 101 | +18 | Higher | -4 | Lower | -22 | Lower |
| PLOC | 85 | **51 ✅** | 89 | +34 | Higher | -4 | Lower | -38 | Lower |
| LLOC | 67 | **48 ✅** | 66 | +19 | Higher | +1 | Higher | -18 | Lower |
| Comments (CLOC) | 3 | **16 ✅** | 0 | -13 | Lower | +3 | Higher | +16 | Higher |
| Blank Lines | **10 ✅** | 16 | 12 | -6 | Lower | -2 | Lower | +4 | Higher |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p03408

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.3436 | 58.7823 | **65.2293 ✅** | +3.5613 | Higher | -2.8858 | Lower | -6.4470 | Lower |
| MI (SEI) | 42.9667 | 38.1182 | **46.2231 ✅** | +4.8485 | Higher | -3.2564 | Lower | -8.1049 | Lower |
| Cyclomatic (sum) | **9 ✅** | 12 | 10 | -3 | Lower | -1 | Lower | +2 | Higher |
| Cognitive (sum) | 9 | **8 ✅** | 15 | +1 | Higher | -6 | Lower | -7 | Lower |
| Halstead Volume | 1489.9780 | 1915.3477 | **1264.8957 ✅** | -425.3697 | Lower | +225.0823 | Higher | +650.4521 | Higher |
| Halstead Effort | 50339.9722 | 73038.5940 | **41437.9827 ✅** | -22698.6219 | Lower | +8901.9894 | Higher | +31600.6113 | Higher |
| SLOC | 69 | 76 | **60 ✅** | -7 | Lower | +9 | Higher | +16 | Higher |
| PLOC | 47 | 55 | **41 ✅** | -8 | Lower | +6 | Higher | +14 | Higher |
| LLOC | 40 | 40 | **33 ✅** | 0 | Same | +7 | Higher | +7 | Higher |
| Comments (CLOC) | 10 | **11 ✅** | 8 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 12 | **10 ✅** | 11 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p03414

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 22.5303 | **55.0325 ✅** | 21.6739 | -32.5022 | Lower | +0.8564 | Higher | +33.3586 | Higher |
| MI (SEI) | -37.4946 | **40.4984 ✅** | -30.2677 | -77.9930 | Lower | -7.2269 | Lower | +70.7661 | Higher |
| Cyclomatic (sum) | 56 | **18 ✅** | 55 | +38 | Higher | +1 | Higher | -37 | Lower |
| Cognitive (sum) | 64 | **22 ✅** | 68 | +42 | Higher | -4 | Lower | -46 | Lower |
| Halstead Volume | 9133.6363 | **3021.1229 ✅** | 9612.8717 | +6112.5134 | Higher | -479.2354 | Lower | -6591.7488 | Lower |
| Halstead Effort | 1185797.9553 | **76132.2969 ✅** | 1359620.5405 | +1109665.6583 | Higher | -173822.5852 | Lower | -1283488.2435 | Lower |
| SLOC | 231 | **76 ✅** | 243 | +155 | Higher | -12 | Lower | -167 | Lower |
| PLOC | 211 | **49 ✅** | 223 | +162 | Higher | -12 | Lower | -174 | Lower |
| LLOC | 159 | **28 ✅** | 164 | +131 | Higher | -5 | Lower | -136 | Lower |
| Comments (CLOC) | 0 | **19 ✅** | 3 | -19 | Lower | -3 | Lower | +16 | Higher |
| Blank Lines | 20 | **10 ✅** | 20 | +10 | Higher | 0 | Same | -10 | Lower |
| Functions | 9 | **2 ✅** | 9 | +7 | Higher | 0 | Same | -7 | Lower |
| Closures | 5 | **0 ✅** | 4 | +5 | Higher | +1 | Higher | -4 | Lower |
| Exits | 9 | **3 ✅** | 9 | +6 | Higher | 0 | Same | -6 | Lower |

## p03434

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.8253 | **68.3419 ✅** | 57.0411 | -10.5166 | Lower | +0.7842 | Higher | +11.3008 | Higher |
| MI (SEI) | 21.6441 | **55.9460 ✅** | 7.9158 | -34.3018 | Lower | +13.7284 | Higher | +48.0302 | Higher |
| Cyclomatic (sum) | 13 | **8 ✅** | 13 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 14 | **0 ✅** | 14 | +14 | Higher | 0 | Same | -14 | Lower |
| Halstead Volume | 2393.5869 | **1995.9665 ✅** | 2459.0831 | +397.6204 | Higher | -65.4962 | Lower | -463.1166 | Lower |
| Halstead Effort | 126957.1439 | **28346.7563 ✅** | 133754.1808 | +98610.3876 | Higher | -6797.0369 | Lower | -105407.4245 | Lower |
| SLOC | 74 | **44 ✅** | 77 | +30 | Higher | -3 | Lower | -33 | Lower |
| PLOC | 60 | **25 ✅** | 63 | +35 | Higher | -3 | Lower | -38 | Lower |
| LLOC | 55 | **11 ✅** | 57 | +44 | Higher | -2 | Lower | -46 | Lower |
| Comments (CLOC) | 2 | **9 ✅** | 0 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | 14 | **10 ✅** | 14 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.0313 ✅** | 69.7446 | 67.2668 | +4.2867 | Higher | +6.7645 | Higher | +2.4778 | Higher |
| MI (SEI) | **56.2083 ✅** | 52.6794 | 46.1951 | +3.5289 | Higher | +10.0132 | Higher | +6.4843 | Higher |
| Cyclomatic (sum) | **10 ✅** | 12 | 13 | -2 | Lower | -3 | Lower | -1 | Lower |
| Cognitive (sum) | **10 ✅** | 12 | 21 | -2 | Lower | -11 | Lower | -9 | Lower |
| Halstead Volume | **965.8751 ✅** | 1111.7954 | 1321.1178 | -145.9202 | Lower | -355.2426 | Lower | -209.3224 | Lower |
| Halstead Effort | 36654.9610 | **33334.0075 ✅** | 72661.4773 | +3320.9535 | Higher | -36006.5162 | Lower | -39327.4697 | Lower |
| SLOC | **38 ✅** | 46 | 50 | -8 | Lower | -12 | Lower | -4 | Lower |
| PLOC | **28 ✅** | 33 | 40 | -5 | Lower | -12 | Lower | -7 | Lower |
| LLOC | **20 ✅** | 23 | 29 | -3 | Lower | -9 | Lower | -6 | Lower |
| Comments (CLOC) | 4 | **6 ✅** | 5 | -2 | Lower | -1 | Lower | +1 | Higher |
| Blank Lines | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p03448

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.2874 | **75.4037 ✅** | 75.2874 | -0.1163 | Lower | 0 | Same | +0.1163 | Higher |
| MI (SEI) | 57.6132 | **73.5596 ✅** | 57.6132 | -15.9464 | Lower | 0 | Same | +15.9464 | Higher |
| Cyclomatic (sum) | 6 | 11 | 6 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 10 | 12 | 10 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 905.4263 | **709.7261 ✅** | 905.4263 | +195.7002 | Higher | 0 | Same | -195.7002 | Lower |
| Halstead Effort | 20330.9358 | **17820.2969 ✅** | 20330.9358 | +2510.6389 | Higher | 0 | Same | -2510.6389 | Lower |
| SLOC | 38 | 38 | 38 | 0 | Same | 0 | Same | 0 | Same |
| PLOC | 26 | **25 ✅** | 26 | +1 | Higher | 0 | Same | -1 | Lower |
| LLOC | 20 | **16 ✅** | 20 | +4 | Higher | 0 | Same | -4 | Lower |
| Comments (CLOC) | 4 | **13 ✅** | 4 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 8 | **6 ✅** | 8 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03454

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.1573 | **156.5144 ✅** | 51.4867 | -105.3571 | Lower | -0.3294 | Lower | +105.0277 | Higher |
| MI (SEI) | 0.6490 | **150.2034 ✅** | 1.1242 | -149.5544 | Lower | -0.4752 | Lower | +149.0793 | Higher |
| Cyclomatic (sum) | 25 | **1 ✅** | 25 | +24 | Higher | 0 | Same | -24 | Lower |
| Cognitive (sum) | 40 | **0 ✅** | 40 | +40 | Higher | 0 | Same | -40 | Lower |
| Halstead Volume | 2958.0284 | **15.5098 ✅** | 2877.1150 | +2942.5186 | Higher | +80.9133 | Higher | -2861.6052 | Lower |
| Halstead Effort | 250459.3753 | **15.5098 ✅** | 254504.7994 | +250443.8656 | Higher | -4045.4241 | Lower | -254489.2896 | Lower |
| SLOC | 88 | **1 ✅** | 87 | +87 | Higher | +1 | Higher | -86 | Lower |
| PLOC | 72 | **1 ✅** | 72 | +71 | Higher | 0 | Same | -71 | Lower |
| LLOC | 53 | **1 ✅** | 53 | +52 | Higher | 0 | Same | -52 | Lower |
| Comments (CLOC) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Blank Lines | 16 | **0 ✅** | 15 | +16 | Higher | +1 | Higher | -15 | Lower |
| Functions | 5 | **0 ✅** | 5 | +5 | Higher | 0 | Same | -5 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **0 ✅** | 4 | +4 | Higher | 0 | Same | -4 | Lower |

## p03463

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.3831 | **156.5144 ✅** | 87.6670 | -77.1312 | Lower | -8.2839 | Lower | +68.8474 | Higher |
| MI (SEI) | 70.0125 | **150.2034 ✅** | 74.0856 | -80.1909 | Lower | -4.0730 | Lower | +76.1179 | Higher |
| Cyclomatic (sum) | 5 | **1 ✅** | 3 | +4 | Higher | +2 | Higher | -2 | Lower |
| Cognitive (sum) | 3 | **0 ✅** | 2 | +3 | Higher | +1 | Higher | -2 | Lower |
| Halstead Volume | 668.1505 | **15.5098 ✅** | 606.6998 | +652.6407 | Higher | +61.4507 | Higher | -591.1900 | Lower |
| Halstead Effort | 11160.5875 | **15.5098 ✅** | 6875.9311 | +11145.0777 | Higher | +4284.6564 | Higher | -6860.4213 | Lower |
| SLOC | 33 | **1 ✅** | 21 | +32 | Higher | +12 | Higher | -20 | Lower |
| PLOC | 22 | **1 ✅** | 15 | +21 | Higher | +7 | Higher | -14 | Lower |
| LLOC | 13 | **1 ✅** | 10 | +12 | Higher | +3 | Higher | -9 | Lower |
| Comments (CLOC) | **6 ✅** | 0 | 2 | +6 | Higher | +4 | Higher | -2 | Lower |
| Blank Lines | 6 | **0 ✅** | 4 | +6 | Higher | +2 | Higher | -4 | Lower |
| Functions | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p03473

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 90.2396 | 84.2326 | **95.3084 ✅** | +6.0070 | Higher | -5.0688 | Lower | -11.0758 | Lower |
| MI (SEI) | 91.7263 | 84.8132 | **96.1394 ✅** | +6.9130 | Higher | -4.4131 | Lower | -11.3262 | Lower |
| Cyclomatic (sum) | 4 | 5 | **2 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Cognitive (sum) | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Halstead Volume | 353.9205 | 438.6884 | **281.7629 ✅** | -84.7679 | Lower | +72.1576 | Higher | +156.9255 | Higher |
| Halstead Effort | 3843.9702 | 5834.5555 | **2403.7897 ✅** | -1990.5853 | Lower | +1440.1805 | Higher | +3430.7658 | Higher |
| SLOC | 21 | 28 | **17 ✅** | -7 | Lower | +4 | Higher | +11 | Higher |
| PLOC | 13 | 17 | **8 ✅** | -4 | Lower | +5 | Higher | +9 | Higher |
| LLOC | 5 | 8 | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Comments (CLOC) | 6 | **9 ✅** | 4 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | 3 | 3 | 5 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03478

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.3493 | 67.0141 | **75.8779 ✅** | +2.3352 | Higher | -6.5286 | Lower | -8.8638 | Lower |
| MI (SEI) | 52.5503 | 54.5395 | **55.8996 ✅** | -1.9891 | Lower | -3.3493 | Lower | -1.3602 | Lower |
| Cyclomatic (sum) | 8 | 13 | **5 ✅** | -5 | Lower | +3 | Higher | +8 | Higher |
| Cognitive (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 1104.2397 | 2065.3963 | **999.7647 ✅** | -961.1566 | Lower | +104.4750 | Higher | +1065.6316 | Higher |
| Halstead Effort | 27784.0958 | 42877.6273 | **25194.0708 ✅** | -15093.5315 | Lower | +2590.0250 | Higher | +17683.5565 | Higher |
| SLOC | 50 | 44 | **36 ✅** | +6 | Higher | +14 | Higher | +8 | Higher |
| PLOC | 34 | 35 | **24 ✅** | -1 | Lower | +10 | Higher | +11 | Higher |
| LLOC | 23 | **19 ✅** | 20 | +4 | Higher | +3 | Higher | -1 | Lower |
| Comments (CLOC) | 7 | **9 ✅** | 3 | -2 | Lower | +4 | Higher | +6 | Higher |
| Blank Lines | 10 | **6 ✅** | 9 | +4 | Higher | +1 | Higher | -3 | Lower |
| Functions | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p03495

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.7967 | **66.4288 ✅** | 57.2791 | -10.6321 | Lower | -1.4824 | Lower | +9.1497 | Higher |
| MI (SEI) | 6.3241 | **51.7262 ✅** | 8.1572 | -45.4021 | Lower | -1.8331 | Lower | +43.5690 | Higher |
| Cyclomatic (sum) | 15 | **5 ✅** | 12 | +10 | Higher | +3 | Higher | -7 | Lower |
| Cognitive (sum) | 14 | **1 ✅** | 11 | +13 | Higher | +3 | Higher | -10 | Lower |
| Halstead Volume | 2538.4720 | 2510.9320 | **2455.3253 ✅** | +27.5400 | Higher | +83.1467 | Higher | +55.6068 | Higher |
| Halstead Effort | 114378.8258 | **40717.8164 ✅** | 125022.5077 | +73661.0094 | Higher | -10643.6819 | Lower | -84304.6913 | Lower |
| SLOC | 80 | **48 ✅** | 77 | +32 | Higher | +3 | Higher | -29 | Lower |
| PLOC | 64 | **32 ✅** | 61 | +32 | Higher | +3 | Higher | -29 | Lower |
| LLOC | 54 | **19 ✅** | 56 | +35 | Higher | -2 | Lower | -37 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 0 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 16 | **9 ✅** | 16 | +7 | Higher | 0 | Same | -7 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03544

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **84.0229 ✅** | 73.0013 | 36.1964 | +11.0215 | Higher | +47.8265 | Higher | +36.8049 | Higher |
| MI (SEI) | **77.3408 ✅** | 65.1270 | -1.4408 | +12.2138 | Higher | +78.7816 | Higher | +66.5678 | Higher |
| Cyclomatic (sum) | **4 ✅** | 9 | 39 | -5 | Lower | -35 | Lower | -30 | Lower |
| Cognitive (sum) | **2 ✅** | 3 | 34 | -1 | Lower | -32 | Lower | -31 | Lower |
| Halstead Volume | **601.3779 ✅** | 1337.3217 | 4568.0469 | -735.9439 | Lower | -3966.6691 | Lower | -3230.7252 | Lower |
| Halstead Effort | **12242.3353 ✅** | 25031.1743 | 440144.7580 | -12788.8390 | Lower | -427902.4227 | Lower | -415113.5837 | Lower |
| SLOC | **26 ✅** | 37 | 158 | -11 | Lower | -132 | Lower | -121 | Lower |
| PLOC | **16 ✅** | 29 | 138 | -13 | Lower | -122 | Lower | -109 | Lower |
| LLOC | **14 ✅** | 16 | 96 | -2 | Lower | -82 | Lower | -80 | Lower |
| Comments (CLOC) | 5 | 9 | 9 | -4 | Lower | -4 | Lower | 0 | Same |
| Blank Lines | 6 | **5 ✅** | 18 | +1 | Higher | -12 | Lower | -13 | Lower |
| Functions | **1 ✅** | 2 | 11 | -1 | Lower | -10 | Lower | -9 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 3 | 12 | -3 | Lower | -12 | Lower | -9 | Lower |

## p03562

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.1980 | **62.4941 ✅** | 42.2924 | -20.2961 | Lower | -0.0944 | Lower | +20.2017 | Higher |
| MI (SEI) | -11.5638 | **47.0403 ✅** | 0.5959 | -58.6042 | Lower | -12.1598 | Lower | +46.4444 | Higher |
| Cyclomatic (sum) | 32 | **11 ✅** | 31 | +21 | Higher | +1 | Higher | -20 | Lower |
| Cognitive (sum) | 52 | **3 ✅** | 50 | +49 | Higher | +2 | Higher | -47 | Lower |
| Halstead Volume | 4747.7680 | **2402.6499 ✅** | 4747.7680 | +2345.1181 | Higher | 0 | Same | -2345.1181 | Lower |
| Halstead Effort | 476696.1104 | **46336.8196 ✅** | 475029.3408 | +430359.2908 | Higher | +1666.7696 | Higher | -428692.5212 | Lower |
| SLOC | 119 | **57 ✅** | 120 | +62 | Higher | -1 | Lower | -63 | Lower |
| PLOC | 105 | **34 ✅** | 105 | +71 | Higher | 0 | Same | -71 | Lower |
| LLOC | 84 | **18 ✅** | 86 | +66 | Higher | -2 | Lower | -68 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 3 | -11 | Lower | -3 | Lower | +8 | Higher |
| Blank Lines | 14 | **12 ✅** | 14 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p03568

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **82.2804 ✅** | 70.1463 | 79.8360 | +12.1341 | Higher | +2.4444 | Higher | -9.6897 | Lower |
| MI (SEI) | 67.6939 | 56.4960 | **74.9586 ✅** | +11.1978 | Higher | -7.2647 | Lower | -18.4626 | Lower |
| Cyclomatic (sum) | **4 ✅** | 8 | 5 | -4 | Lower | -1 | Lower | +3 | Higher |
| Cognitive (sum) | 3 | 3 | 4 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | **667.4306 ✅** | 1898.4956 | 674.0397 | -1231.0650 | Lower | -6.6091 | Lower | +1224.4559 | Higher |
| Halstead Effort | **11360.2244 ✅** | 25937.0660 | 13787.1752 | -14576.8416 | Lower | -2426.9508 | Lower | +12149.8908 | Higher |
| SLOC | **28 ✅** | 40 | 32 | -12 | Lower | -4 | Lower | +8 | Higher |
| PLOC | **18 ✅** | 25 | 21 | -7 | Lower | -3 | Lower | +4 | Higher |
| LLOC | 14 | 14 | 17 | 0 | Same | -3 | Lower | -3 | Lower |
| Comments (CLOC) | 3 | 7 | **8 ✅** | -4 | Lower | -5 | Lower | -1 | Lower |
| Blank Lines | 7 | 8 | **6 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03592

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **82.8382 ✅** | 81.2669 | 52.4839 | +1.5713 | Higher | +30.3543 | Higher | +28.7830 | Higher |
| MI (SEI) | 65.1470 | **74.4405 ✅** | 2.2574 | -9.2935 | Lower | +62.8896 | Higher | +72.1831 | Higher |
| Cyclomatic (sum) | 5 | 5 | 22 | 0 | Same | -17 | Lower | -17 | Lower |
| Cognitive (sum) | 6 | 6 | 18 | 0 | Same | -12 | Lower | -12 | Lower |
| Halstead Volume | 722.5705 | **695.6089 ✅** | 2278.7366 | +26.9616 | Higher | -1556.1660 | Lower | -1583.1276 | Lower |
| Halstead Effort | 12236.5749 | **11779.9863 ✅** | 100176.7659 | +456.5886 | Higher | -87940.1910 | Lower | -88396.7796 | Lower |
| SLOC | **26 ✅** | 29 | 92 | -3 | Lower | -66 | Lower | -63 | Lower |
| PLOC | 19 | 19 | 77 | 0 | Same | -58 | Lower | -58 | Lower |
| LLOC | 13 | 13 | 47 | 0 | Same | -34 | Lower | -34 | Lower |
| Comments (CLOC) | 2 | **6 ✅** | 0 | -4 | Lower | +2 | Higher | +6 | Higher |
| Blank Lines | 5 | **4 ✅** | 15 | +1 | Higher | -10 | Lower | -11 | Lower |
| Functions | 1 | 1 | 9 | 0 | Same | -8 | Lower | -8 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 7 | 0 | Same | -6 | Lower | -6 | Lower |

## p03608

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **52.7973 ✅** | 44.5735 | 52.7356 | +8.2238 | Higher | +0.0618 | Higher | -8.1621 | Lower |
| MI (SEI) | 2.5060 | **9.9343 ✅** | 2.3151 | -7.4283 | Lower | +0.1909 | Higher | +7.6192 | Higher |
| Cyclomatic (sum) | 20 | 19 | 19 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 30 | **23 ✅** | 30 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | **3230.1321 ✅** | 5072.9141 | 3291.4250 | -1842.7820 | Lower | -61.2928 | Lower | +1781.4891 | Higher |
| Halstead Effort | 176556.0868 | **170123.7979 ✅** | 176988.8978 | +6432.2889 | Higher | -432.8110 | Lower | -6865.0999 | Lower |
| SLOC | **83 ✅** | 121 | 84 | -38 | Lower | -1 | Lower | +37 | Higher |
| PLOC | **70 ✅** | 96 | 72 | -26 | Lower | -2 | Lower | +24 | Higher |
| LLOC | 54 | **43 ✅** | 55 | +11 | Higher | -1 | Lower | -12 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 0 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 13 | 20 | **12 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| Functions | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |

## p03626

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.5812 | 66.6051 | **71.1478 ✅** | -7.0239 | Lower | -11.5666 | Lower | -4.5427 | Lower |
| MI (SEI) | 33.6880 | **57.7535 ✅** | 27.8601 | -24.0655 | Lower | +5.8279 | Higher | +29.8934 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 15 | **3 ✅** | 15 | +12 | Higher | 0 | Same | -12 | Lower |
| Halstead Volume | 2910.9097 | 1871.7674 | **1195.9332 ✅** | +1039.1423 | Higher | +1714.9765 | Higher | +675.8342 | Higher |
| Halstead Effort | 73069.7736 | **29413.4870 ✅** | 42479.5472 | +43656.2867 | Higher | +30590.2265 | Higher | -13066.0602 | Lower |
| SLOC | 66 | 50 | **43 ✅** | +16 | Higher | +23 | Higher | +7 | Higher |
| PLOC | 50 | **33 ✅** | 36 | +17 | Higher | +14 | Higher | -3 | Lower |
| LLOC | 40 | **13 ✅** | 34 | +27 | Higher | +6 | Higher | -21 | Lower |
| Comments (CLOC) | 6 | **14 ✅** | 0 | -8 | Lower | +6 | Higher | +14 | Higher |
| Blank Lines | 10 | 9 | **7 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p03632

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.3532 | **75.4670 ✅** | 71.3845 | -3.1138 | Lower | +0.9687 | Higher | +4.0824 | Higher |
| MI (SEI) | 51.4563 | **68.6842 ✅** | 53.0406 | -17.2279 | Lower | -1.5844 | Lower | +15.6435 | Higher |
| Cyclomatic (sum) | 14 | 9 | **8 ✅** | +5 | Higher | +6 | Higher | +1 | Higher |
| Cognitive (sum) | 13 | **4 ✅** | 7 | +9 | Higher | +6 | Higher | -3 | Lower |
| Halstead Volume | 1214.2813 | **832.3579 ✅** | 1111.8397 | +381.9233 | Higher | +102.4416 | Higher | -279.4817 | Lower |
| Halstead Effort | 48484.5161 | **24948.8336 ✅** | 32675.7325 | +23535.6825 | Higher | +15808.7836 | Higher | -7726.8989 | Lower |
| SLOC | 37 | 37 | 44 | 0 | Same | -7 | Lower | -7 | Lower |
| PLOC | **26 ✅** | 28 | 32 | -2 | Lower | -6 | Lower | -4 | Lower |
| LLOC | 19 | **16 ✅** | 26 | +3 | Higher | -7 | Lower | -10 | Lower |
| Comments (CLOC) | 3 | **9 ✅** | 5 | -6 | Lower | -2 | Lower | +4 | Higher |
| Blank Lines | 8 | **4 ✅** | 8 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03646

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.9290 | **72.9861 ✅** | 68.6520 | -9.0571 | Lower | -4.7230 | Lower | +4.3341 | Higher |
| MI (SEI) | 32.8509 | **65.9256 ✅** | 40.1713 | -33.0747 | Lower | -7.3203 | Lower | +25.7543 | Higher |
| Cyclomatic (sum) | 12 | **7 ✅** | 11 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 17 | **3 ✅** | 19 | +14 | Higher | -2 | Lower | -16 | Lower |
| Halstead Volume | 2466.7319 | **987.2503 ✅** | 1340.8642 | +1479.4816 | Higher | +1125.8678 | Higher | -353.6138 | Lower |
| Halstead Effort | 61546.0726 | **24064.2270 ✅** | 39332.0155 | +37481.8457 | Higher | +22214.0571 | Higher | -15267.7886 | Lower |
| SLOC | 51 | **42 ✅** | 47 | +9 | Higher | +4 | Higher | -5 | Lower |
| PLOC | 44 | **26 ✅** | 43 | +18 | Higher | +1 | Higher | -17 | Lower |
| LLOC | 27 | **19 ✅** | 32 | +8 | Higher | -5 | Lower | -13 | Lower |
| Comments (CLOC) | 2 | **11 ✅** | 2 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 7 | 5 | **4 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03647

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.6681 ✅** | 65.3102 | 67.7131 | +5.3579 | Higher | +2.9550 | Higher | -2.4029 | Lower |
| MI (SEI) | 55.9546 | 51.5169 | **56.7177 ✅** | +4.4377 | Higher | -0.7631 | Lower | -5.2008 | Lower |
| Cyclomatic (sum) | 10 | **8 ✅** | 11 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | 18 | **5 ✅** | 16 | +13 | Higher | +2 | Higher | -11 | Lower |
| Halstead Volume | **1089.0677 ✅** | 1889.1896 | 1104.7124 | -800.1219 | Lower | -15.6447 | Lower | +784.4772 | Higher |
| Halstead Effort | **27952.7373 ✅** | 30832.8059 | 30158.6483 | -2880.0686 | Lower | -2205.9110 | Lower | +674.1576 | Higher |
| SLOC | **45 ✅** | 54 | 53 | -9 | Lower | -8 | Lower | +1 | Higher |
| PLOC | **32 ✅** | 33 | 39 | -1 | Lower | -7 | Lower | -6 | Lower |
| LLOC | 25 | **14 ✅** | 29 | +11 | Higher | -4 | Lower | -15 | Lower |
| Comments (CLOC) | 7 | 11 | **12 ✅** | -4 | Lower | -5 | Lower | -1 | Lower |
| Blank Lines | **8 ✅** | 10 | 9 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03701

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.5424 | 71.8865 | **75.9305 ✅** | -0.3441 | Lower | -4.3881 | Lower | -4.0440 | Lower |
| MI (SEI) | 56.4717 | **61.8802 ✅** | 47.3234 | -5.4085 | Lower | +9.1482 | Higher | +14.5568 | Higher |
| Cyclomatic (sum) | 8 | 8 | **7 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 12 | 16 | **9 ✅** | -4 | Lower | +3 | Higher | +7 | Higher |
| Halstead Volume | **878.2464 ✅** | 878.9685 | 905.9093 | -0.7221 | Lower | -27.6630 | Lower | -26.9408 | Lower |
| Halstead Effort | **18578.2882 ✅** | 21267.2159 | 28083.1891 | -2688.9277 | Lower | -9504.9009 | Lower | -6815.9733 | Lower |
| SLOC | 47 | 46 | **36 ✅** | +1 | Higher | +11 | Higher | +10 | Higher |
| PLOC | 33 | 32 | **30 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| LLOC | 22 | **20 ✅** | 27 | +2 | Higher | -5 | Lower | -7 | Lower |
| Comments (CLOC) | 7 | **10 ✅** | 1 | -3 | Lower | +6 | Higher | +9 | Higher |
| Blank Lines | 8 | 7 | **6 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03709

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.4767 | **70.7799 ✅** | 19.6382 | -33.3032 | Lower | +17.8386 | Higher | +51.1417 | Higher |
| MI (SEI) | -18.3751 | **59.7484 ✅** | -40.8525 | -78.1235 | Lower | +22.4774 | Higher | +100.6009 | Higher |
| Cyclomatic (sum) | 32 | **7 ✅** | 64 | +25 | Higher | -32 | Lower | -57 | Lower |
| Cognitive (sum) | 50 | **5 ✅** | 82 | +45 | Higher | -32 | Lower | -77 | Lower |
| Halstead Volume | 6359.6202 | **2061.1094 ✅** | 9549.8070 | +4298.5108 | Higher | -3190.1867 | Lower | -7488.6975 | Lower |
| Halstead Effort | 675266.5610 | **44127.4755 ✅** | 1143854.6559 | +631139.0855 | Higher | -468588.0949 | Lower | -1099727.1804 | Lower |
| SLOC | 145 | **38 ✅** | 243 | +107 | Higher | -98 | Lower | -205 | Lower |
| PLOC | 124 | **27 ✅** | 206 | +97 | Higher | -82 | Lower | -179 | Lower |
| LLOC | 106 | **15 ✅** | 125 | +91 | Higher | -19 | Lower | -110 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 0 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 21 | **7 ✅** | 37 | +14 | Higher | -16 | Lower | -30 | Lower |
| Functions | 4 | **2 ✅** | 16 | +2 | Higher | -12 | Lower | -14 | Lower |
| Closures | 4 | **0 ✅** | 3 | +4 | Higher | +1 | Higher | -3 | Lower |
| Exits | 3 | **1 ✅** | 15 | +2 | Higher | -12 | Lower | -14 | Lower |

## p03719

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **71.0125 ✅** | 68.6392 | 34.6462 | +2.3733 | Higher | +36.3664 | Higher | +33.9930 | Higher |
| MI (SEI) | **52.3525 ✅** | 44.5860 | -5.7555 | +7.7665 | Higher | +58.1079 | Higher | +50.3414 | Higher |
| Cyclomatic (sum) | 9 | 9 | 42 | 0 | Same | -33 | Lower | -33 | Lower |
| Cognitive (sum) | **3 ✅** | 4 | 38 | -1 | Lower | -35 | Lower | -34 | Lower |
| Halstead Volume | **2135.0477 ✅** | 2247.3115 | 4708.9256 | -112.2639 | Lower | -2573.8780 | Lower | -2461.6141 | Lower |
| Halstead Effort | **39284.8769 ✅** | 46891.3829 | 438473.4230 | -7606.5060 | Lower | -399188.5461 | Lower | -391582.0401 | Lower |
| SLOC | **36 ✅** | 41 | 165 | -5 | Lower | -129 | Lower | -124 | Lower |
| PLOC | **23 ✅** | 28 | 145 | -5 | Lower | -122 | Lower | -117 | Lower |
| LLOC | **9 ✅** | 12 | 94 | -3 | Lower | -85 | Lower | -82 | Lower |
| Comments (CLOC) | 4 | 3 | **7 ✅** | +1 | Higher | -3 | Lower | -4 | Lower |
| Blank Lines | **9 ✅** | 10 | 18 | -1 | Lower | -9 | Lower | -8 | Lower |
| Functions | 1 | 1 | 12 | 0 | Same | -11 | Lower | -11 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **0 ✅** | 1 | 13 | -1 | Lower | -13 | Lower | -12 | Lower |

## p03722

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.8728 | **156.5144 ✅** | 61.3245 | -96.6416 | Lower | -1.4517 | Lower | +95.1899 | Higher |
| MI (SEI) | 33.8074 | **150.2034 ✅** | 13.9935 | -116.3960 | Lower | +19.8139 | Higher | +136.2099 | Higher |
| Cyclomatic (sum) | 12 | **1 ✅** | 12 | +11 | Higher | 0 | Same | -11 | Lower |
| Cognitive (sum) | 17 | **0 ✅** | 17 | +17 | Higher | 0 | Same | -17 | Lower |
| Halstead Volume | 2006.5142 | **15.5098 ✅** | 2006.5142 | +1991.0044 | Higher | 0 | Same | -1991.0044 | Lower |
| Halstead Effort | 82449.4911 | **15.5098 ✅** | 82449.4911 | +82433.9814 | Higher | 0 | Same | -82433.9814 | Lower |
| SLOC | 70 | **1 ✅** | 64 | +69 | Higher | +6 | Higher | -63 | Lower |
| PLOC | 54 | **1 ✅** | 54 | +53 | Higher | 0 | Same | -53 | Lower |
| LLOC | 38 | **1 ✅** | 38 | +37 | Higher | 0 | Same | -37 | Lower |
| Comments (CLOC) | **6 ✅** | 0 | 0 | +6 | Higher | +6 | Higher | 0 | Same |
| Blank Lines | 11 | **0 ✅** | 10 | +11 | Higher | +1 | Higher | -10 | Lower |
| Functions | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03736

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.5106 | **69.5132 ✅** | 58.1265 | -12.0026 | Lower | -0.6159 | Lower | +11.3868 | Higher |
| MI (SEI) | 8.3894 | **64.3224 ✅** | 9.3797 | -55.9330 | Lower | -0.9903 | Lower | +54.9426 | Higher |
| Cyclomatic (sum) | 11 | **4 ✅** | 12 | +7 | Higher | -1 | Lower | -8 | Lower |
| Cognitive (sum) | 8 | **2 ✅** | 8 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | 3025.6756 | **2369.7948 ✅** | 2807.3159 | +655.8808 | Higher | +218.3597 | Higher | -437.5211 | Lower |
| Halstead Effort | 171310.8699 | **43047.6489 ✅** | 159037.7087 | +128263.2210 | Higher | +12273.1612 | Higher | -115990.0598 | Lower |
| SLOC | 72 | **41 ✅** | 70 | +31 | Higher | +2 | Higher | -29 | Lower |
| PLOC | 59 | **28 ✅** | 58 | +31 | Higher | +1 | Higher | -30 | Lower |
| LLOC | 52 | **22 ✅** | 49 | +30 | Higher | +3 | Higher | -27 | Lower |
| Comments (CLOC) | 0 | **14 ✅** | 0 | -14 | Lower | 0 | Same | +14 | Higher |
| Blank Lines | 13 | **6 ✅** | 12 | +7 | Higher | +1 | Higher | -6 | Lower |
| Functions | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |

## p03762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.8647 | **77.4549 ✅** | 70.0917 | -11.5902 | Lower | -4.2270 | Lower | +7.3632 | Higher |
| MI (SEI) | 46.7031 | **78.3814 ✅** | 54.7158 | -31.6783 | Lower | -8.0127 | Lower | +23.6656 | Higher |
| Cyclomatic (sum) | 9 | 11 | **6 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Cognitive (sum) | 6 | 4 | 4 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 1624.6253 | 2286.1808 | **1452.2088 ✅** | -661.5555 | Lower | +172.4165 | Higher | +833.9720 | Higher |
| Halstead Effort | 74461.9938 | **46295.1609 ✅** | 52642.5694 | +28166.8329 | Higher | +21819.4244 | Higher | -6347.4084 | Lower |
| SLOC | 54 | **23 ✅** | 45 | +31 | Higher | +9 | Higher | -22 | Lower |
| PLOC | 36 | **27 ✅** | 29 | +9 | Higher | +7 | Higher | -2 | Lower |
| LLOC | 29 | **14 ✅** | 25 | +15 | Higher | +4 | Higher | -11 | Lower |
| Comments (CLOC) | 7 | **9 ✅** | 7 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 11 | **-12 ✅** | 9 | +23 | Higher | +2 | Higher | -21 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 1 | 4 | **0 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |

## p03766

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.7550 ✅** | 62.3365 | 67.4865 | +6.4185 | Higher | +1.2685 | Higher | -5.1500 | Lower |
| MI (SEI) | 40.3473 | **44.1218 ✅** | 38.0253 | -3.7745 | Lower | +2.3220 | Higher | +6.0965 | Higher |
| Cyclomatic (sum) | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | **1718.9052 ✅** | 2588.5709 | 1794.2061 | -869.6657 | Lower | -75.3010 | Lower | +794.3647 | Higher |
| Halstead Effort | 83616.4189 | **61582.1006 ✅** | 85080.0971 | +22034.3184 | Higher | -1463.6782 | Lower | -23497.9966 | Lower |
| SLOC | **45 ✅** | 57 | 48 | -12 | Lower | -3 | Lower | +9 | Higher |
| PLOC | **36 ✅** | 46 | 40 | -10 | Lower | -4 | Lower | +6 | Higher |
| LLOC | 28 | **22 ✅** | 30 | +6 | Higher | -2 | Lower | -8 | Lower |
| Comments (CLOC) | 2 | **9 ✅** | 2 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 9 | 10 | **8 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p03797

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.1577 | 75.7535 | **83.2057 ✅** | +1.4041 | Higher | -6.0480 | Lower | -7.4522 | Lower |
| MI (SEI) | 58.4305 | **62.8478 ✅** | 60.0939 | -4.4174 | Lower | -1.6635 | Lower | +2.7539 | Higher |
| Cyclomatic (sum) | 6 | 7 | **5 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 893.5940 | **730.4165 ✅** | 760.7724 | +163.1775 | Higher | +132.8216 | Higher | -30.3559 | Lower |
| Halstead Effort | 20503.0179 | 17815.8102 | **14884.6771 ✅** | +2687.2077 | Higher | +5618.3408 | Higher | +2931.1331 | Higher |
| SLOC | 34 | 39 | **25 ✅** | -5 | Lower | +9 | Higher | +14 | Higher |
| PLOC | 24 | 26 | **20 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |
| LLOC | 20 | **10 ✅** | 17 | +10 | Higher | +3 | Higher | -7 | Lower |
| Comments (CLOC) | 3 | **6 ✅** | 1 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | 7 | 7 | **4 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03808

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.2635 | **66.6620 ✅** | 62.8645 | -5.3985 | Lower | -1.6009 | Lower | +3.7976 | Higher |
| MI (SEI) | 44.4903 | **53.6667 ✅** | 16.3171 | -9.1764 | Lower | +28.1733 | Higher | +37.3496 | Higher |
| Cyclomatic (sum) | 13 | **7 ✅** | 13 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 23 | **3 ✅** | 22 | +20 | Higher | +1 | Higher | -19 | Lower |
| Halstead Volume | **1684.0086 ✅** | 2197.5537 | 1839.4060 | -513.5451 | Lower | -155.3974 | Lower | +358.1477 | Higher |
| Halstead Effort | 126300.6460 | **34698.2169 ✅** | 136965.0030 | +91602.4291 | Higher | -10664.3570 | Lower | -102266.7861 | Lower |
| SLOC | 67 | **48 ✅** | 59 | +19 | Higher | +8 | Higher | -11 | Lower |
| PLOC | 50 | **29 ✅** | 53 | +21 | Higher | -3 | Lower | -24 | Lower |
| LLOC | 44 | **14 ✅** | 44 | +30 | Higher | 0 | Same | -30 | Lower |
| Comments (CLOC) | **12 ✅** | 10 | 0 | +2 | Higher | +12 | Higher | +10 | Higher |
| Blank Lines | 9 | 9 | **6 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03815

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 87.1140 | 69.0832 | 87.1140 | +18.0308 | Higher | 0 | Same | -18.0308 | Lower |
| MI (SEI) | 80.4875 | 60.3420 | 80.4875 | +20.1455 | Higher | 0 | Same | -20.1455 | Lower |
| Cyclomatic (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 486.2570 | 1842.7568 | 486.2570 | -1356.4998 | Lower | 0 | Same | +1356.4998 | Higher |
| Halstead Effort | 8314.9948 | 31983.7747 | 8314.9948 | -23668.7800 | Lower | 0 | Same | +23668.7800 | Higher |
| SLOC | 23 | 45 | 23 | -22 | Lower | 0 | Same | +22 | Higher |
| PLOC | 14 | 22 | 14 | -8 | Lower | 0 | Same | +8 | Higher |
| LLOC | 5 | 8 | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Comments (CLOC) | 4 | **12 ✅** | 4 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 5 | 12 | 5 | -7 | Lower | 0 | Same | +7 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03817

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **85.8417 ✅** | 82.5217 | 68.0867 | +3.3199 | Higher | +17.7549 | Higher | +14.4350 | Higher |
| MI (SEI) | **80.4852 ✅** | 78.3487 | 42.3728 | +2.1365 | Higher | +38.1125 | Higher | +35.9759 | Higher |
| Cyclomatic (sum) | **4 ✅** | 5 | 13 | -1 | Lower | -9 | Lower | -8 | Lower |
| Cognitive (sum) | **3 ✅** | 4 | 9 | -1 | Lower | -6 | Lower | -5 | Lower |
| Halstead Volume | **478.9735 ✅** | 546.4703 | 1128.4003 | -67.4968 | Lower | -649.4268 | Lower | -581.9300 | Lower |
| Halstead Effort | **7367.0690 ✅** | 11475.8764 | 33739.1699 | -4108.8074 | Lower | -26372.1008 | Lower | -22263.2935 | Lower |
| SLOC | **25 ✅** | 29 | 50 | -4 | Lower | -25 | Lower | -21 | Lower |
| PLOC | **15 ✅** | 17 | 39 | -2 | Lower | -24 | Lower | -22 | Lower |
| LLOC | **6 ✅** | 14 | 10 | -8 | Lower | -4 | Lower | +4 | Higher |
| Comments (CLOC) | 5 | **7 ✅** | 3 | -2 | Lower | +2 | Higher | +4 | Higher |
| Blank Lines | 5 | 5 | 10 | 0 | Same | -5 | Lower | -5 | Lower |
| Functions | 1 | 1 | 7 | 0 | Same | -6 | Lower | -6 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 6 | 0 | Same | -6 | Lower | -6 | Lower |

## p03837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.8107 ✅** | 57.6490 | 57.2874 | +1.1617 | Higher | +1.5233 | Higher | +0.3616 | Higher |
| MI (SEI) | 33.1327 | **44.0588 ✅** | 34.6137 | -10.9261 | Lower | -1.4809 | Lower | +9.4451 | Higher |
| Cyclomatic (sum) | 16 | 18 | 16 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 19 | **17 ✅** | 19 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | **2363.5935 ✅** | 2972.9110 | 2531.7408 | -609.3176 | Lower | -168.1474 | Lower | +441.1702 | Higher |
| Halstead Effort | 126022.5055 | **89321.2455 ✅** | 147516.0997 | +36701.2600 | Higher | -21493.5942 | Lower | -58194.8542 | Lower |
| SLOC | 67 | **65 ✅** | 72 | +2 | Higher | -5 | Lower | -7 | Lower |
| PLOC | 56 | **45 ✅** | 54 | +11 | Higher | +2 | Higher | -9 | Lower |
| LLOC | 46 | **24 ✅** | 44 | +22 | Higher | +2 | Higher | -20 | Lower |
| Comments (CLOC) | 6 | **16 ✅** | 9 | -10 | Lower | -3 | Lower | +7 | Higher |
| Blank Lines | 11 | **8 ✅** | 11 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p03838

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.6015 ✅** | 70.6411 | 73.4533 | +3.9604 | Higher | +1.1482 | Higher | -2.8122 | Lower |
| MI (SEI) | 54.1908 | **62.7762 ✅** | 57.8282 | -8.5854 | Lower | -3.6374 | Lower | +4.9480 | Higher |
| Cyclomatic (sum) | 9 | **6 ✅** | 9 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 12 | **3 ✅** | 12 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | **983.0848 ✅** | 1746.1916 | 1128.2488 | -763.1067 | Lower | -145.1639 | Lower | +617.9428 | Higher |
| Halstead Effort | **23929.1781 ✅** | 25067.5499 | 30021.2278 | -1138.3718 | Lower | -6092.0497 | Lower | -4953.6779 | Lower |
| SLOC | **37 ✅** | 41 | 38 | -4 | Lower | -1 | Lower | +3 | Higher |
| PLOC | 31 | **26 ✅** | 32 | +5 | Higher | -1 | Lower | -6 | Lower |
| LLOC | 10 | **8 ✅** | 27 | +2 | Higher | -17 | Lower | -19 | Lower |
| Comments (CLOC) | 3 | **11 ✅** | 5 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | 3 | 7 | 3 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03845

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.1412 | 69.2699 | **76.2526 ✅** | -1.1287 | Lower | -8.1114 | Lower | -6.9826 | Lower |
| MI (SEI) | 52.7148 | 64.1523 | **66.7529 ✅** | -11.4375 | Lower | -14.0381 | Lower | -2.6006 | Lower |
| Cyclomatic (sum) | 8 | 6 | **5 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Cognitive (sum) | 6 | 5 | **3 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Halstead Volume | 1483.5314 | 1824.1858 | **1015.5980 ✅** | -340.6544 | Lower | +467.9334 | Higher | +808.5877 | Higher |
| Halstead Effort | 44926.2758 | 36410.3285 | **23519.1124 ✅** | +8515.9474 | Higher | +21407.1634 | Higher | +12891.2160 | Higher |
| SLOC | 49 | 44 | **35 ✅** | +5 | Higher | +14 | Higher | +9 | Higher |
| PLOC | 34 | 27 | **21 ✅** | +7 | Higher | +13 | Higher | +6 | Higher |
| LLOC | 21 | **16 ✅** | 18 | +5 | Higher | +3 | Higher | -2 | Lower |
| Comments (CLOC) | 8 | **15 ✅** | 7 | -7 | Lower | +1 | Higher | +8 | Higher |
| Blank Lines | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03865

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.4519 | **87.7214 ✅** | 78.8675 | -14.2695 | Lower | -5.4156 | Lower | +8.8539 | Higher |
| MI (SEI) | 61.0677 | **90.2542 ✅** | 69.9032 | -29.1865 | Lower | -8.8355 | Lower | +20.3510 | Higher |
| Cyclomatic (sum) | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 1904.7974 | **378.9225 ✅** | 1620.7450 | +1525.8749 | Higher | +284.0524 | Higher | -1241.8225 | Lower |
| Halstead Effort | 26349.6976 | **4067.1017 ✅** | 25548.9963 | +22282.5959 | Higher | +800.7013 | Higher | -21481.8946 | Lower |
| SLOC | 34 | **24 ✅** | 26 | +10 | Higher | +8 | Higher | -2 | Lower |
| PLOC | 22 | **15 ✅** | 19 | +7 | Higher | +3 | Higher | -4 | Lower |
| LLOC | 8 | 9 | **7 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Comments (CLOC) | 6 | **8 ✅** | 5 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 6 | **4 ✅** | 5 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03899

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.7669 ✅** | 61.6370 | 60.6257 | +1.1299 | Higher | +2.1412 | Higher | +1.0113 | Higher |
| MI (SEI) | 35.5989 | **41.3435 ✅** | 12.9854 | -5.7446 | Lower | +22.6135 | Higher | +28.3582 | Higher |
| Cyclomatic (sum) | 11 | 11 | 12 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 11 | **9 ✅** | 14 | +2 | Higher | -3 | Lower | -5 | Lower |
| Halstead Volume | **2047.6051 ✅** | 2544.5442 | 2085.3160 | -496.9391 | Lower | -37.7109 | Lower | +459.2282 | Higher |
| Halstead Effort | 86596.6305 | **58369.3610 ✅** | 93162.9000 | +28227.2695 | Higher | -6566.2695 | Lower | -34793.5390 | Lower |
| SLOC | 59 | 59 | 66 | 0 | Same | -7 | Lower | -7 | Lower |
| PLOC | 45 | **37 ✅** | 54 | +8 | Higher | -9 | Lower | -17 | Lower |
| LLOC | 34 | **24 ✅** | 39 | +10 | Higher | -5 | Lower | -15 | Lower |
| Comments (CLOC) | 4 | **8 ✅** | 0 | -4 | Lower | +4 | Higher | +8 | Higher |
| Blank Lines | **10 ✅** | 15 | 12 | -5 | Lower | -2 | Lower | +3 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03901

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.0977 | **71.1148 ✅** | 43.1108 | -5.0172 | Lower | +22.9868 | Higher | +28.0040 | Higher |
| MI (SEI) | 52.1025 | **59.5576 ✅** | 15.6547 | -7.4551 | Lower | +36.4478 | Higher | +43.9030 | Higher |
| Cyclomatic (sum) | **5 ✅** | 10 | 15 | -5 | Lower | -10 | Lower | -5 | Lower |
| Cognitive (sum) | **1 ✅** | 4 | 11 | -3 | Lower | -10 | Lower | -7 | Lower |
| Halstead Volume | 2356.4643 | **933.2624 ✅** | 4374.2751 | +1423.2019 | Higher | -2017.8108 | Lower | -3441.0127 | Lower |
| Halstead Effort | 44369.4636 | **21133.8776 ✅** | 147558.8804 | +23235.5860 | Higher | -103189.4168 | Lower | -126425.0028 | Lower |
| SLOC | 50 | **46 ✅** | 147 | +4 | Higher | -97 | Lower | -101 | Lower |
| PLOC | 32 | **29 ✅** | 110 | +3 | Higher | -78 | Lower | -81 | Lower |
| LLOC | 14 | 14 | 70 | 0 | Same | -56 | Lower | -56 | Lower |
| Comments (CLOC) | 10 | 9 | **21 ✅** | +1 | Higher | -11 | Lower | -12 | Lower |
| Blank Lines | 10 | **8 ✅** | 19 | +2 | Higher | -9 | Lower | -11 | Lower |
| Functions | 2 | 2 | 5 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |

## p03914

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.9343 | 77.4806 | **78.1198 ✅** | -3.5463 | Lower | -4.1855 | Lower | -0.6392 | Lower |
| MI (SEI) | 49.7726 | **76.5836 ✅** | 37.5113 | -26.8110 | Lower | +12.2613 | Higher | +39.0724 | Higher |
| Cyclomatic (sum) | 7 | 7 | **5 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 8 | 8 | **6 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | 1451.8387 | **803.4553 ✅** | 1274.1099 | +648.3834 | Higher | +177.7288 | Higher | -470.6546 | Lower |
| Halstead Effort | 48570.6048 | **25710.5695 ✅** | 40036.4540 | +22860.0353 | Higher | +8534.1508 | Higher | -14325.8845 | Lower |
| SLOC | 35 | 34 | **29 ✅** | +1 | Higher | +6 | Higher | +5 | Higher |
| PLOC | 27 | **23 ✅** | 24 | +4 | Higher | +3 | Higher | -1 | Lower |
| LLOC | 24 | **14 ✅** | 22 | +10 | Higher | +2 | Higher | -8 | Lower |
| Comments (CLOC) | 2 | **12 ✅** | 0 | -10 | Lower | +2 | Higher | +12 | Higher |
| Blank Lines | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03929

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.0696 | **73.0845 ✅** | 67.1079 | -6.0149 | Lower | -0.0383 | Lower | +5.9766 | Higher |
| MI (SEI) | 51.2751 | **60.8431 ✅** | 43.3694 | -9.5680 | Lower | +7.9058 | Higher | +17.4738 | Higher |
| Cyclomatic (sum) | 14 | **8 ✅** | 14 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 16 | **3 ✅** | 24 | +13 | Higher | -8 | Lower | -21 | Lower |
| Halstead Volume | 1234.2767 | 1790.2172 | **1153.2892 ✅** | -555.9405 | Lower | +80.9876 | Higher | +636.9280 | Higher |
| Halstead Effort | 51839.6219 | **32321.2037 ✅** | 47625.5997 | +19518.4182 | Higher | +4214.0223 | Higher | -15304.3960 | Lower |
| SLOC | 51 | **34 ✅** | 52 | +17 | Higher | -1 | Lower | -18 | Lower |
| PLOC | 34 | **22 ✅** | 41 | +12 | Higher | -7 | Lower | -19 | Lower |
| LLOC | 28 | **11 ✅** | 31 | +17 | Higher | -3 | Lower | -20 | Lower |
| Comments (CLOC) | **8 ✅** | 6 | 4 | +2 | Higher | +4 | Higher | +2 | Higher |
| Blank Lines | 9 | **6 ✅** | 7 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03932

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.2462 | **51.2964 ✅** | 36.7372 | -9.0502 | Lower | +5.5090 | Higher | +14.5592 | Higher |
| MI (SEI) | 17.6836 | **31.9330 ✅** | 12.1423 | -14.2494 | Lower | +5.5413 | Higher | +19.7907 | Higher |
| Cyclomatic (sum) | 22 | **18 ✅** | 27 | +4 | Higher | -5 | Lower | -9 | Lower |
| Cognitive (sum) | 24 | **14 ✅** | 26 | +10 | Higher | -2 | Lower | -12 | Lower |
| Halstead Volume | 4720.3371 | **2993.1168 ✅** | 5781.4729 | +1727.2203 | Higher | -1061.1359 | Lower | -2788.3561 | Lower |
| Halstead Effort | 439620.7254 | **184263.7532 ✅** | 469278.4282 | +255356.9723 | Higher | -29657.7027 | Lower | -285014.6750 | Lower |
| SLOC | 137 | **96 ✅** | 168 | +41 | Higher | -31 | Lower | -72 | Lower |
| PLOC | 100 | **60 ✅** | 114 | +40 | Higher | -14 | Lower | -54 | Lower |
| LLOC | 65 | **47 ✅** | 74 | +18 | Higher | -9 | Lower | -27 | Lower |
| Comments (CLOC) | 24 | 19 | **34 ✅** | +5 | Higher | -10 | Lower | -15 | Lower |
| Blank Lines | 20 | **19 ✅** | 27 | +1 | Higher | -7 | Lower | -8 | Lower |
| Functions | 5 | **2 ✅** | 7 | +3 | Higher | -2 | Lower | -5 | Lower |
| Closures | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |

## p03951

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.4698 | 68.8068 | **70.2676 ✅** | -6.3369 | Lower | -7.7978 | Lower | -1.4608 | Lower |
| MI (SEI) | 41.4795 | **53.7769 ✅** | 51.2778 | -12.2974 | Lower | -9.7983 | Lower | +2.4991 | Higher |
| Cyclomatic (sum) | 10 | 9 | 9 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 14 | **7 ✅** | 14 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 2666.0598 | 1248.8106 | **1229.4484 ✅** | +1417.2492 | Higher | +1436.6114 | Higher | +19.3622 | Higher |
| Halstead Effort | 63652.1770 | **35965.7449 ✅** | 52743.3352 | +27686.4321 | Higher | +10908.8418 | Higher | -16777.5902 | Lower |
| SLOC | 56 | 49 | **45 ✅** | +7 | Higher | +11 | Higher | +4 | Higher |
| PLOC | 43 | 37 | **36 ✅** | +6 | Higher | +7 | Higher | +1 | Higher |
| LLOC | 34 | **27 ✅** | 34 | +7 | Higher | 0 | Same | -7 | Lower |
| Comments (CLOC) | 7 | **8 ✅** | 5 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 9 | **6 ✅** | 7 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03977

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **66.5342 ✅** | 62.6397 | 59.3961 | +3.8945 | Higher | +7.1381 | Higher | +3.2435 | Higher |
| MI (SEI) | **54.9659 ✅** | 47.0468 | 35.7966 | +7.9191 | Higher | +19.1693 | Higher | +11.2502 | Higher |
| Cyclomatic (sum) | 13 | **9 ✅** | 14 | +4 | Higher | -1 | Lower | -5 | Lower |
| Cognitive (sum) | 13 | **5 ✅** | 19 | +8 | Higher | -6 | Lower | -14 | Lower |
| Halstead Volume | **1196.7415 ✅** | 2552.3877 | 1692.9565 | -1355.6463 | Lower | -496.2150 | Lower | +859.4312 | Higher |
| Halstead Effort | **44527.0357 ✅** | 45579.8343 | 81098.0779 | -1052.7986 | Lower | -36571.0421 | Lower | -35518.2435 | Lower |
| SLOC | **54 ✅** | 57 | 74 | -3 | Lower | -20 | Lower | -17 | Lower |
| PLOC | **32 ✅** | 38 | 56 | -6 | Lower | -24 | Lower | -18 | Lower |
| LLOC | 25 | **17 ✅** | 49 | +8 | Higher | -24 | Lower | -32 | Lower |
| Comments (CLOC) | **12 ✅** | 11 | 8 | +1 | Higher | +4 | Higher | +3 | Higher |
| Blank Lines | 11 | **9 ✅** | 10 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | 1 | **0 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |

## p03996

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.7300 | **61.6199 ✅** | 55.2359 | -9.8900 | Lower | -3.5059 | Lower | +6.3841 | Higher |
| MI (SEI) | 18.6058 | **51.7034 ✅** | 24.0773 | -33.0976 | Lower | -5.4715 | Lower | +27.6261 | Higher |
| Cyclomatic (sum) | 16 | **12 ✅** | 14 | +4 | Higher | +2 | Higher | -2 | Lower |
| Cognitive (sum) | 16 | **6 ✅** | 16 | +10 | Higher | 0 | Same | -10 | Lower |
| Halstead Volume | 3945.0891 | **2201.5254 ✅** | 2736.6190 | +1743.5637 | Higher | +1208.4701 | Higher | -535.0936 | Lower |
| Halstead Effort | 106955.7491 | **45791.7275 ✅** | 128954.8250 | +61164.0217 | Higher | -21999.0759 | Lower | -83163.0975 | Lower |
| SLOC | 88 | **61 ✅** | 82 | +27 | Higher | +6 | Higher | -21 | Lower |
| PLOC | 72 | **32 ✅** | 66 | +40 | Higher | +6 | Higher | -34 | Lower |
| LLOC | 50 | **13 ✅** | 56 | +37 | Higher | -6 | Lower | -43 | Lower |
| Comments (CLOC) | 5 | **18 ✅** | 5 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | 11 | 15 | 11 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | **1 ✅** | 3 | 2 | -2 | Lower | -1 | Lower | +1 | Higher |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **1 ✅** | 4 | 2 | -3 | Lower | -1 | Lower | +2 | Higher |

## p04005

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.2965 | 70.4802 | **74.0672 ✅** | -3.1837 | Lower | -6.7707 | Lower | -3.5870 | Lower |
| MI (SEI) | 49.2054 | 57.6445 | **65.8395 ✅** | -8.4391 | Lower | -16.6341 | Lower | -8.1950 | Lower |
| Cyclomatic (sum) | 10 | 8 | 8 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | 8 | **3 ✅** | 9 | +5 | Higher | -1 | Lower | -6 | Lower |
| Halstead Volume | 2082.7695 | 2088.9266 | **966.4985 ✅** | -6.1571 | Lower | +1116.2710 | Higher | +1122.4282 | Higher |
| Halstead Effort | 38275.9944 | 38703.1684 | **29342.8933 ✅** | -427.1740 | Lower | +8933.1012 | Higher | +9360.2752 | Higher |
| SLOC | 45 | **38 ✅** | 39 | +7 | Higher | +6 | Higher | -1 | Lower |
| PLOC | 33 | 28 | **26 ✅** | +5 | Higher | +7 | Higher | +2 | Higher |
| LLOC | 18 | **11 ✅** | 22 | +7 | Higher | -4 | Lower | -11 | Lower |
| Comments (CLOC) | 6 | 7 | **9 ✅** | -1 | Lower | -3 | Lower | -2 | Lower |
| Blank Lines | 6 | **4 ✅** | 7 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p04013

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.3322 ✅** | 62.5831 | 63.1401 | +0.7492 | Higher | +0.1921 | Higher | -0.5571 | Lower |
| MI (SEI) | **41.9849 ✅** | 41.1269 | 37.9505 | +0.8580 | Higher | +4.0344 | Higher | +3.1764 | Higher |
| Cyclomatic (sum) | 16 | **14 ✅** | 17 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | 18 | **8 ✅** | 18 | +10 | Higher | 0 | Same | -10 | Lower |
| Halstead Volume | **1939.9695 ✅** | 2753.2512 | 2166.1671 | -813.2817 | Lower | -226.1976 | Lower | +587.0841 | Higher |
| Halstead Effort | 113811.5421 | **75761.8779 ✅** | 144050.1117 | +38049.6642 | Higher | -30238.5696 | Lower | -68288.2338 | Lower |
| SLOC | 54 | 52 | 52 | +2 | Higher | +2 | Higher | 0 | Same |
| PLOC | 39 | **36 ✅** | 39 | +3 | Higher | 0 | Same | -3 | Lower |
| LLOC | 34 | **22 ✅** | 33 | +12 | Higher | +1 | Higher | -11 | Lower |
| Comments (CLOC) | 6 | 6 | 4 | 0 | Same | +2 | Higher | +2 | Higher |
| Blank Lines | 9 | 11 | 9 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p04020

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.9454 | 65.2830 | **80.3904 ✅** | +10.6624 | Higher | -4.4450 | Lower | -15.1074 | Lower |
| MI (SEI) | 62.2104 | 54.1871 | **65.1708 ✅** | +8.0233 | Higher | -2.9603 | Lower | -10.9836 | Lower |
| Cyclomatic (sum) | 7 | 10 | **6 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 986.1761 | 2352.8276 | **878.6965 ✅** | -1366.6516 | Lower | +107.4796 | Higher | +1474.1311 | Higher |
| Halstead Effort | **32050.7223 ✅** | 49200.4844 | 32951.1186 | -17149.7621 | Lower | -900.3963 | Lower | +16249.3659 | Higher |
| SLOC | 35 | 49 | **28 ✅** | -14 | Lower | +7 | Higher | +21 | Higher |
| PLOC | 23 | 33 | 23 | -10 | Lower | 0 | Same | +10 | Higher |
| LLOC | **21 ✅** | 23 | 22 | -2 | Lower | -1 | Lower | +1 | Higher |
| Comments (CLOC) | 5 | **12 ✅** | 3 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | 7 | 9 | **4 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p04022

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 32.0279 | **61.9191 ✅** | 2.8395 | -29.8913 | Lower | +29.1884 | Higher | +59.0796 | Higher |
| MI (SEI) | -25.6252 | **45.8421 ✅** | -57.0226 | -71.4673 | Lower | +31.3973 | Higher | +102.8646 | Higher |
| Cyclomatic (sum) | 38 | **14 ✅** | 91 | +24 | Higher | -53 | Lower | -77 | Lower |
| Cognitive (sum) | 86 | **13 ✅** | 108 | +73 | Higher | -22 | Lower | -95 | Lower |
| Halstead Volume | 5353.0785 | **2002.9983 ✅** | 13012.5398 | +3350.0803 | Higher | -7659.4612 | Lower | -11009.5415 | Lower |
| Halstead Effort | 499620.6638 | **47244.6332 ✅** | 1535725.2110 | +452376.0306 | Higher | -1036104.5472 | Lower | -1488480.5778 | Lower |
| SLOC | 197 | **60 ✅** | 423 | +137 | Higher | -226 | Lower | -363 | Lower |
| PLOC | 173 | **41 ✅** | 370 | +132 | Higher | -197 | Lower | -329 | Lower |
| LLOC | 152 | **23 ✅** | 245 | +129 | Higher | -93 | Lower | -222 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 2 | -11 | Lower | -2 | Lower | +9 | Higher |
| Blank Lines | 24 | **11 ✅** | 51 | +13 | Higher | -27 | Lower | -40 | Lower |
| Functions | 3 | 3 | 36 | 0 | Same | -33 | Lower | -33 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 4 | **3 ✅** | 34 | +1 | Higher | -30 | Lower | -31 | Lower |

## p04024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.8968 | **68.0093 ✅** | 40.7559 | -28.1124 | Lower | -0.8590 | Lower | +27.2534 | Higher |
| MI (SEI) | -14.3745 | **59.1530 ✅** | -13.1352 | -73.5276 | Lower | -1.2393 | Lower | +72.2882 | Higher |
| Cyclomatic (sum) | 37 | **12 ✅** | 37 | +25 | Higher | 0 | Same | -25 | Lower |
| Cognitive (sum) | 51 | **9 ✅** | 51 | +42 | Higher | 0 | Same | -42 | Lower |
| Halstead Volume | 4607.4144 | **1197.1354 ✅** | 4202.8909 | +3410.2790 | Higher | +404.5235 | Higher | -3005.7555 | Lower |
| Halstead Effort | 385998.9395 | **46688.2796 ✅** | 353186.1139 | +339310.6599 | Higher | +32812.8256 | Higher | -306497.8343 | Lower |
| SLOC | 129 | **50 ✅** | 126 | +79 | Higher | +3 | Higher | -76 | Lower |
| PLOC | 108 | **34 ✅** | 108 | +74 | Higher | 0 | Same | -74 | Lower |
| LLOC | 79 | **20 ✅** | 81 | +59 | Higher | -2 | Lower | -61 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 0 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | 21 | **7 ✅** | 18 | +14 | Higher | +3 | Higher | -11 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 9 | **3 ✅** | 9 | +6 | Higher | 0 | Same | -6 | Lower |

## p04030

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.9524 ✅** | 68.1667 | 69.8779 | +9.7857 | Higher | +8.0745 | Higher | -1.7112 | Lower |
| MI (SEI) | **73.4266 ✅** | 60.0344 | 60.5564 | +13.3922 | Higher | +12.8702 | Higher | -0.5220 | Lower |
| Cyclomatic (sum) | **6 ✅** | 7 | 8 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 8 | 8 | 11 | 0 | Same | -3 | Lower | -3 | Lower |
| Halstead Volume | **589.3307 ✅** | 1878.6742 | 1851.0467 | -1289.3435 | Lower | -1261.7160 | Lower | +27.6276 | Higher |
| Halstead Effort | **9454.9146 ✅** | 37573.4848 | 29431.6422 | -28118.5702 | Lower | -19976.7276 | Lower | +8141.8426 | Higher |
| SLOC | **37 ✅** | 46 | 41 | -9 | Lower | -4 | Lower | +5 | Higher |
| PLOC | **23 ✅** | 30 | 31 | -7 | Lower | -8 | Lower | -1 | Lower |
| LLOC | **17 ✅** | 20 | 18 | -3 | Lower | -1 | Lower | +2 | Higher |
| Comments (CLOC) | 10 | **13 ✅** | 10 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 7 | 7 | **6 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## Overall Averages

| Metric | Avg specs_and_code | Avg specs_only | Avg code_only | Avg Delta (s+c - s) | Change | Avg Delta (s+c - c) | Change | Avg Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.9578 | **66.8603 ✅** | 61.3572 | -4.9025 | Lower | 0.6006 | Higher | 5.5031 | Higher |
| MI (SEI) | 35.0447 | **51.4637 ✅** | 31.0557 | -16.4190 | Lower | 3.9890 | Higher | 20.4081 | Higher |
| Cyclomatic (sum) | 15.9905 | **12.1395 ✅** | 16.9976 | 3.8511 | Higher | -1.0071 | Lower | -4.8582 | Lower |
| Cognitive (sum) | 20.5296 | **10.4279 ✅** | 22.1277 | 10.1017 | Higher | -1.5981 | Lower | -11.6998 | Lower |
| Halstead Volume | 2429.5888 | **1931.2939 ✅** | 2544.8807 | 498.2949 | Higher | -115.2919 | Lower | -613.5867 | Lower |
| Halstead Effort | 151280.3799 | **55025.7909 ✅** | 190998.2466 | 96254.5891 | Higher | -39717.8667 | Lower | -135972.4558 | Lower |
| SLOC | 70.3073 | **54.7494 ✅** | 76.3026 | 15.5579 | Higher | -5.9953 | Lower | -21.5532 | Lower |
| PLOC | 57.1513 | **39.6690 ✅** | 63.7943 | 17.4823 | Higher | -6.6430 | Lower | -24.1253 | Lower |
| LLOC | 40.3428 | **21.3026 ✅** | 47.7116 | 19.0402 | Higher | -7.3688 | Lower | -26.4090 | Lower |
| Comments (CLOC) | 4.7447 | **9.4232 ✅** | 3.6383 | -4.6785 | Lower | 1.1064 | Higher | 5.7849 | Higher |
| Blank Lines | 9.9102 | **8.4444 ✅** | 10.1844 | 1.4657 | Higher | -0.2742 | Lower | -1.7400 | Lower |
| Functions | 2.3522 | **2.2222 ✅** | 2.8960 | 0.1300 | Higher | -0.5437 | Lower | -0.6738 | Lower |
| Closures | 0.6076 | 0.6217 | **0.4397 ✅** | -0.0142 | Lower | 0.1678 | Higher | 0.1820 | Higher |
| Exits | **1.7329 ✅** | 1.8487 | 2.0520 | -0.1158 | Lower | -0.3191 | Lower | -0.2033 | Lower |

