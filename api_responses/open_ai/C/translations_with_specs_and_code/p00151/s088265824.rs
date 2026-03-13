// translated from the provided C program according to the SPEC.
// Behavior: read test cases (n followed by n whitespace-separated row-strings).
// For each n>0, compute the maximum contiguous run of '1' along the sequences
// actually scanned by the original C program (rows, columns, certain diagonals,
// and two passes that scan column suffixes). Print one integer per line.
// If n == 0 or input ends, terminate.
//
// Notes:
// - This Rust translation follows the SPEC's description of the scanning behavior,
//   including the two column-suffix passes that duplicate work in the original C.
// - To avoid panics on short/missing input rows, out-of-range accesses are treated
//   as non-'1' (which is safer than crashing). The SPEC documents that behavior
//   for malformed input is undefined in the C program; this translation keeps
//   sensible, safe semantics while preserving the intended outputs for well-formed input.

use std::io::{self, Read};

fn main() {
    // Read all stdin and iterate over whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();

    loop {
        // Read n
        let n_token = match tokens.next() {
            Some(t) => t,
            None => break, // EOF
        };
        let n: usize = match n_token.parse() {
            Ok(v) => v,
            Err(_) => break, // malformed integer -> stop (mirrors unspecified behavior)
        };
        if n == 0 {
            break;
        }

        // Read n rows (whitespace-separated strings). Rows may be longer; only first n chars used.
        let mut grid: Vec<Vec<u8>> = Vec::with_capacity(n);
        for _ in 0..n {
            let row_bytes = match tokens.next() {
                Some(s) => s.as_bytes().to_vec(),
                None => Vec::new(), // missing row: treat as empty (will be treated as non-'1's)
            };
            grid.push(row_bytes);
        }

        let mut max_ones: usize = 0;
        let mut ones: usize;

        // 1) Rows: for each row i, scan columns 0..n-1
        for i in 0..n {
            ones = 0;
            for j in 0..n {
                let ch = grid.get(i).and_then(|r| r.get(j)).copied().unwrap_or(b'0');
                if ch == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
            }
            if ones > max_ones {
                max_ones = ones;
            }
        }

        // 2) Columns: for each column j, scan rows 0..n-1
        for j in 0..n {
            ones = 0;
            for i in 0..n {
                let ch = grid.get(i).and_then(|r| r.get(j)).copied().unwrap_or(b'0');
                if ch == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
            }
            if ones > max_ones {
                max_ones = ones;
            }
        }

        // 3) Down-left diagonals starting from top-row columns:
        //    for start column j in 0..n: positions (r=0,c=j),(r=1,c=j-1),... until c < 0
        for start_j in 0..n {
            ones = 0;
            let mut i = 0usize;
            let mut k = start_j as isize;
            while k != -1 {
                let col = k as usize;
                let ch = grid.get(i).and_then(|r| r.get(col)).copied().unwrap_or(b'0');
                if ch == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
                i += 1;
                k -= 1;
            }
            if ones > max_ones {
                max_ones = ones;
            }
        }

        // 4) Additional pass: for i = 1..n-1 examine grid[k][i] for k = i..n-1
        //    (this mirrors the C code's column-suffix scan)
        for col_i in 1..n {
            ones = 0;
            let mut k = col_i;
            while k != n {
                let ch = grid.get(k).and_then(|r| r.get(col_i)).copied().unwrap_or(b'0');
                if ch == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
                k += 1;
            }
            if ones > max_ones {
                max_ones = ones;
            }
        }

        // 5) Down-right diagonals starting from top-row columns:
        //    for start column j in 0..n: positions (r=0,c=j),(r=1,c=j+1),... until c >= n
        for start_j in 0..n {
            ones = 0;
            let mut i = 0usize;
            let mut k = start_j;
            while k != n {
                let ch = grid.get(i).and_then(|r| r.get(k)).copied().unwrap_or(b'0');
                if ch == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
                i += 1;
                k += 1;
            }
            if ones > max_ones {
                max_ones = ones;
            }
        }

        // 6) Repeated additional pass (same as 4), as in the C program:
        for col_i in 1..n {
            ones = 0;
            let mut k = col_i;
            while k != n {
                let ch = grid.get(k).and_then(|r| r.get(col_i)).copied().unwrap_or(b'0');
                if ch == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
                k += 1;
            }
            if ones > max_ones {
                max_ones = ones;
            }
        }

        // Print result for this test case.
        println!("{}", max_ones);
    }
}
