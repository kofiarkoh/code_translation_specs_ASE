// translated_spec.rs
//
// Translation of the provided Specification into Rust.
// Behavior notes:
// - Reads test cases from stdin as whitespace-separated tokens.
// - Each test case: an integer n. If n == 0 the program terminates.
// - Then n strings follow (one per row). Only the first n characters of each
//   string are considered as the grid cells for that row. If a provided row
//   token has fewer than n characters, it's padded with a non-'1' char
//   (NUL) so indexing is always defined in this Rust implementation.
// - For each test case the program prints a single line with the maximum
//   contiguous run length of the character '1' encountered among the specific
//   scans described in the Specification. The scans include rows, columns,
//   diagonals starting from the top row (down-left and down-right), and two
//   identical passes that scan column suffixes (for i=1..n-1, cells
//   (k,i) for k=i..n-1), as described.
//
// This implementation favors safe Rust and makes minimal, nondisruptive choices
// where the Specification left behavior undefined (e.g., short row tokens).
// In well-formed input (rows >= n chars and 0 <= n <= 255) the behavior
// exactly matches the described scanning semantics.

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string and split by whitespace tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we can't read stdin, exit silently (no output).
        return;
    }
    let mut tokens = input.split_whitespace();

    // Helper to get next token; returns None when tokens exhausted.
    loop {
        let n_token = match tokens.next() {
            Some(t) => t,
            None => break, // EOF or no more tokens
        };
        // Parse integer n. The Specification assumes well-formed integer input.
        // We parse as isize to detect zero termination; treat negative the same
        // as zero and terminate (n < 0 is unspecified in spec, so choose narrow).
        let n_val: isize = match n_token.parse() {
            Ok(v) => v,
            Err(_) => break, // malformed integer: the spec leaves this undefined; stop.
        };
        if n_val == 0 {
            break; // termination case
        }
        if n_val < 0 {
            break; // unspecified by spec; choose to stop
        }
        let n = n_val as usize;

        // Read n row tokens. If tokens are missing, use empty string(s) and pad.
        // The Specification assumes there are at least n strings; here we handle
        // missing tokens gracefully by treating missing characters as non-'1'.
        let mut grid: Vec<Vec<char>> = Vec::with_capacity(n);
        for _ in 0..n {
            let tok = tokens.next().unwrap_or("");
            // Take up to first n characters; if shorter, pad with '\0' (non-'1').
            let mut row: Vec<char> = tok.chars().take(n).collect();
            if row.len() < n {
                row.resize(n, '\0');
            }
            grid.push(row);
        }

        // Helper closure: check if grid[r][c] == '1'. Assumes 0 <= r,c < n.
        let is_one = |r: usize, c: usize| -> bool { grid[r][c] == '1' };

        let mut max_ones: usize = 0;

        // 1) Rows: for each row i, sequence grid[i][0..n-1]
        for i in 0..n {
            let mut cur: usize = 0;
            for j in 0..n {
                if is_one(i, j) {
                    cur += 1;
                    if cur > max_ones {
                        max_ones = cur;
                    }
                } else {
                    cur = 0;
                }
            }
        }

        // 2) Columns: for each column j, sequence grid[0..n-1][j]
        for j in 0..n {
            let mut cur: usize = 0;
            for i in 0..n {
                if is_one(i, j) {
                    cur += 1;
                    if cur > max_ones {
                        max_ones = cur;
                    }
                } else {
                    cur = 0;
                }
            }
        }

        // 3) Down-left diagonals starting from top-row columns:
        //    for start column j in 0..n-1: (r=0,c=j),(r=1,c=j-1),... until c < 0 or r >= n
        for start_j in 0..n {
            let mut cur: usize = 0;
            let mut r: usize = 0;
            // c as isize to allow negative test
            let mut c: isize = start_j as isize;
            while r < n && c >= 0 {
                if is_one(r, c as usize) {
                    cur += 1;
                    if cur > max_ones {
                        max_ones = cur;
                    }
                } else {
                    cur = 0;
                }
                r += 1;
                c -= 1;
            }
        }

        // 4) Down-right diagonals starting from top-row columns:
        //    for start column j in 0..n-1: (r=0,c=j),(r=1,c=j+1),... until c >= n or r >= n
        for start_j in 0..n {
            let mut cur: usize = 0;
            let mut r: usize = 0;
            let mut c: usize = start_j;
            while r < n && c < n {
                if is_one(r, c) {
                    cur += 1;
                    if cur > max_ones {
                        max_ones = cur;
                    }
                } else {
                    cur = 0;
                }
                r += 1;
                c += 1;
            }
        }

        // 5) Additional scans: for outer index i = 1..n-1, examine contiguous runs
        //    over cells (r=k, c=i) for k=i..n-1 (a suffix of column i).
        //    This is performed twice as per the Specification.
        for _pass in 0..2 {
            for i in 1..n {
                let mut cur: usize = 0;
                for k in i..n {
                    if is_one(k, i) {
                        cur += 1;
                        if cur > max_ones {
                            max_ones = cur;
                        }
                    } else {
                        cur = 0;
                    }
                }
            }
        }

        // Print result for this test case.
        println!("{}", max_ones);
    }
}
