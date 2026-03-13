// max_submatrix.rs
// Translated from the provided C program according to the SPEC.
// Computes the maximum-sum contiguous submatrix of an n x n integer matrix.
// Input: n followed by n rows of n integers (whitespace separated).
// Output: single integer (max sum) followed by newline.
//
// Notes:
// - The SPEC treats C int semantics as authoritative; here we use i32 for matrix
//   elements and running sums, matching typical 32-bit C int usage. The SPEC
//   assumes inputs/sums stay within that range.
// - Parsing is implemented via whitespace tokenization. The SPEC requires each
//   row on its own line, but token-wise whitespace parsing is compatible with
//   those requirements and is robust for the specified, well-formed inputs.
// - This implementation avoids any unsafe code and follows the algorithm in the
//   SPEC: for every column interval, compute per-row column-interval sums and
//   apply a 1D Kadane-like scan over rows.
//
// Usage: read from stdin, write single integer + newline to stdout.

use std::io::{self, Read};

fn main() {
    // Read entire stdin to a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    // Tokenize by whitespace. SPEC guarantees well-formed tokens.
    let mut toks = input.split_whitespace();

    // Read n.
    let n: usize = match toks.next() {
        Some(t) => t.parse().expect("failed to parse n"),
        None => return, // No input; just exit (SPEC allows undefined behavior; keep simple).
    };

    // According to SPEC, 1 <= n <= 101. We do not enforce it strictly here.
    // Read n*n integers row-major.
    let mut a: Vec<Vec<i32>> = vec![vec![0i32; n]; n];
    for r in 0..n {
        for c in 0..n {
            let tok = toks.next().expect("not enough integers in input");
            a[r][c] = tok.parse::<i32>().expect("failed to parse matrix integer");
        }
    }

    // Build prefix sums over columns: s[r][j] = sum of a[r][0..j-1], j in 0..=n
    let mut s: Vec<Vec<i32>> = vec![vec![0i32; n + 1]; n];
    for r in 0..n {
        for c in 0..n {
            // s[r][c+1] = s[r][c] + a[r][c]
            s[r][c + 1] = s[r][c].wrapping_add(a[r][c]);
        }
    }

    // Initialize answer to first element (s[0][1] == a[0][0]) if possible.
    // SPEC requires n >= 1.
    let mut ans: i32 = s[0][1];

    // For every column interval [c..k-1] (c in 0..n, k in c+1..=n)
    for c in 0..n {
        for k in (c + 1)..=n {
            let mut t: i32 = 0;
            for r in 0..n {
                let seg_sum = s[r][k].wrapping_sub(s[r][c]);
                if t < 0 {
                    t = seg_sum;
                } else {
                    t = t.wrapping_add(seg_sum);
                }
                if t > ans {
                    ans = t;
                }
            }
        }
    }

    // Print result followed by newline.
    println!("{}", ans);
}
