# Rust Metrics Comparison

Source: `r_code/metrics.json`

## p00001

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.4711 | 71.4629 | **79.1514 ✅** | +2.0081 | Higher | -5.6804 | Lower | -7.6885 | Lower |
| MI (SEI) | 68.6319 | **69.2576 ✅** | 65.6136 | -0.6257 | Lower | +3.0184 | Higher | +3.6440 | Higher |
| Cyclomatic (sum) | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 835.7781 | **755.3707 ✅** | 776.9541 | +80.4074 | Higher | +58.8240 | Higher | -21.5834 | Lower |
| Halstead Effort | 18648.2991 | **15539.0552 ✅** | 18011.2093 | +3109.2440 | Higher | +637.0899 | Higher | -2472.1541 | Lower |
| SLOC | 43 | 51 | **31 ✅** | -8 | Lower | +12 | Higher | +20 | Higher |
| PLOC | 24 | 26 | 24 | -2 | Lower | 0 | Same | +2 | Higher |
| LLOC | 22 | 25 | 22 | -3 | Lower | 0 | Same | +3 | Higher |
| Comments (CLOC) | 13 | **20 ✅** | 4 | -7 | Lower | +9 | Higher | +16 | Higher |
| Blank Lines | 6 | 7 | **4 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00002

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.4582 | 68.8915 | **83.7755 ✅** | -0.4334 | Lower | -15.3173 | Lower | -14.8839 | Lower |
| MI (SEI) | 67.6939 | 69.1967 | **77.6062 ✅** | -1.5028 | Lower | -9.9122 | Lower | -8.4095 | Lower |
| Cyclomatic (sum) | 8 | **3 ✅** | 5 | +5 | Higher | +3 | Higher | -2 | Lower |
| Cognitive (sum) | 3 | **1 ✅** | 6 | +2 | Higher | -3 | Lower | -5 | Lower |
| Halstead Volume | 705.4144 | 697.1726 | **681.8169 ✅** | +8.2418 | Higher | +23.5975 | Higher | +15.3557 | Higher |
| Halstead Effort | 10665.8660 | **8784.3745 ✅** | 10800.5993 | +1881.4915 | Higher | -134.7333 | Lower | -2016.2248 | Lower |
| SLOC | 61 | 64 | **25 ✅** | -3 | Lower | +36 | Higher | +39 | Higher |
| PLOC | 25 | 20 | **18 ✅** | +5 | Higher | +7 | Higher | +2 | Higher |
| LLOC | 13 | 13 | **12 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 29 | **34 ✅** | 5 | -5 | Lower | +24 | Higher | +29 | Higher |
| Blank Lines | 8 | 11 | **4 ✅** | -3 | Lower | +4 | Higher | +7 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## Overall Averages

| Metric | Avg specs_and_code | Avg specs_only | Avg code_only | Avg Delta (s+c - s) | Change | Avg Delta (s+c - c) | Change | Avg Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.9646 | 70.1772 | **81.4634 ✅** | 0.7874 | Higher | -10.4988 | Lower | -11.2862 | Lower |
| MI (SEI) | 68.1629 | 69.2272 | **71.6099 ✅** | -1.0642 | Lower | -3.4469 | Lower | -2.3827 | Lower |
| Cyclomatic (sum) | 7.5000 | **4.5000 ✅** | 6 | 3 | Higher | 1.5000 | Higher | -1.5000 | Lower |
| Cognitive (sum) | 4.5000 | **3 ✅** | 6 | 1.5000 | Higher | -1.5000 | Lower | -3 | Lower |
| Halstead Volume | 770.5963 | **726.2717 ✅** | 729.3855 | 44.3246 | Higher | 41.2108 | Higher | -3.1138 | Lower |
| Halstead Effort | 14657.0826 | **12161.7148 ✅** | 14405.9043 | 2495.3677 | Higher | 251.1783 | Higher | -2244.1895 | Lower |
| SLOC | 52 | 57.5000 | **28 ✅** | -5.5000 | Lower | 24 | Higher | 29.5000 | Higher |
| PLOC | 24.5000 | 23 | **21 ✅** | 1.5000 | Higher | 3.5000 | Higher | 2 | Higher |
| LLOC | 17.5000 | 19 | **17 ✅** | -1.5000 | Lower | 0.5000 | Higher | 2 | Higher |
| Comments (CLOC) | 21 | **27 ✅** | 4.5000 | -6 | Lower | 16.5000 | Higher | 22.5000 | Higher |
| Blank Lines | 7 | 9 | **4 ✅** | -2 | Lower | 3 | Higher | 5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0.5000 | 0 | 0 | 0.5000 | Higher | 0.5000 | Higher | 0 | Same |

