// max_submatrix.rs
//
// Translates the provided specification into Rust.
// Computes the maximum sum over all contiguous rectangular submatrices
// of an n x n integer matrix read from stdin.
//
// Notes on fidelity to the specification:
// - Matrix elements and arithmetic use i32 to model C `int`.
// - To emulate C signed-integer wraparound behavior on overflow, all
//   accumulation uses wrapping_add so sums wrap like C (no overflow checks).
// - Parsing is token-based: first token is `n`, then n*n integer tokens.
//   The program will panic on malformed input (per spec: behavior is undefined
//   on malformed input).
//
// This program prints a single integer followed by a newline: the maximum
// submatrix sum.

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    // Tokenize by whitespace. Spec expects specific formatting; we accept
    // whitespace-separated tokens and will panic if input is malformed.
    let mut toks = input.split_whitespace();

    // Parse n (first token). Spec guarantees it's a non-negative decimal integer.
    let n_tok = toks.next().expect("missing n");
    let n: usize = n_tok.parse().expect("failed to parse n as integer");

    // Read n*n integers for the matrix row-major.
    // Spec: each matrix integer fits in C int; we use i32 and allow wraparound
    // in arithmetic by using wrapping_add during sums.
    let mut matrix: Vec<Vec<i32>> = vec![vec![0i32; n]; n];
    for i in 0..n {
        for j in 0..n {
            let tok = toks.next().expect("missing matrix element");
            // Parse as i32. If malformed, this will panic, matching spec's undefined behavior.
            let val: i32 = tok.parse().expect("failed to parse matrix element as i32");
            matrix[i][j] = val;
        }
    }

    // Algorithm: for each pair of column boundaries, accumulate per-row sums
    // and run a 1D Kadane-like scan over rows. Use wrapping_add to emulate C int wrap.
    let mut best: i32 = i32::MIN;

    for left in 0..n {
        // tmp[row] holds sum of elements in row `row` between columns left..=right
        let mut tmp: Vec<i32> = vec![0i32; n];

        for right in left..n {
            // Add column `right` into tmp
            for row in 0..n {
                tmp[row] = tmp[row].wrapping_add(matrix[row][right]);
            }

            // Kadane-like scan over tmp to find max contiguous subarray sum.
            // cur is the running sum; reset to 0 when negative (classic variant).
            // Use wrapping_add to emulate C overflow behavior.
            let mut cur: i32 = 0;
            for row in 0..n {
                cur = cur.wrapping_add(tmp[row]);
                if cur > best {
                    best = cur;
                }
                if cur < 0 {
                    cur = 0;
                }
            }
        }
    }

    // Print result followed by newline.
    println!("{}", best);
}
