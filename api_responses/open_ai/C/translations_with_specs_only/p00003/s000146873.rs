// pythagorean_triplets.rs
//
// Translates the provided specification into Rust:
// - Reads an integer N followed by N triplets (a, b, c) from stdin.
// - For each triplet, determines whether the largest of the three (by >= ties)
//   squared equals the sum of the squares of the other two.
// - Emits one line per processed triplet: either "YES" or "NO".
//
// Implementation notes (kept to preserve semantics from the spec):
// - Uses i32 for integers to represent a typical host "signed int".
// - Arithmetic for squaring and summing uses wrapping operations (wrapping_mul,
//   wrapping_add) to emulate two's-complement wraparound rather than panicking
//   on overflow. The specification indicates overflow is not checked and may be
//   undefined; using wrapping preserves a consistent, C-like wraparound behavior.
// - Ties for the largest value are resolved deterministically using >= checks:
//   if a >= b && a >= c then a is treated as hypotenuse; else if b >= a && b >= c
//   then b is; otherwise c is.
// - Input parsing is permissive: tokens that fail to parse as i32 are skipped.
//   If N is missing or there are fewer than 3*N integers, the program processes
//   as many complete triplets as are available (this is a narrow, defined
//   behavior chosen in place of the unspecified behavior in the original).
// - The program prints exactly one line per processed triplet ("YES" or "NO"),
//   in the same order as triplets appear in input.
//
// This program avoids unsafe code and stays idiomatic Rust while preserving the
// essential observable behavior described in the specification.

use std::io::{self, Read};

fn main() {
    // Read all stdin into a string
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On I/O error, exit silently (no output). This follows the spirit of
        // minimal external behavior in the spec.
        return;
    }

    // Parse whitespace-separated tokens into i32 where possible.
    // Tokens that fail to parse as i32 are skipped.
    let nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if nums.is_empty() {
        // No N present; chosen behavior: do nothing and exit normally.
        return;
    }

    let n = nums[0];
    if n <= 0 {
        // Per spec: if N <= 0, do not process any triplets and produce no output.
        return;
    }

    let n_usize = n as usize;
    // How many complete triplets are available in the input after N?
    let available_triplets = (nums.len().saturating_sub(1)) / 3;
    // Process up to min(N, available_triplets). If fewer than N triplets are
    // available, we stop early (a narrow, defined choice).
    let to_process = n_usize.min(available_triplets);

    for i in 0..to_process {
        let base = 1 + 3 * i;
        let a = nums[base];
        let b = nums[base + 1];
        let c = nums[base + 2];

        // Determine which is the largest with deterministic tie-breaking.
        let (hyp, x, y) = if a >= b && a >= c {
            (a, b, c)
        } else if b >= a && b >= c {
            (b, a, c)
        } else {
            (c, a, b)
        };

        // Use wrapping arithmetic to emulate host signed-int overflow/wraparound.
        let hyp_sq = hyp.wrapping_mul(hyp);
        let sum_sq = x.wrapping_mul(x).wrapping_add(y.wrapping_mul(y));

        if hyp_sq == sum_sq {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
