// translation.rs
//
// Translation of the given Specification to Rust.
// - Reads whitespace-separated integer tokens from stdin (like scanf("%d", &n)).
// - For each integer `n` (assumed to be a 32-bit signed C int, i32), computes:
//     m = floor((600 - n) / n) for positive n,
//     sum = n^3 * sum_{k=1..m} k^2
//   where the set of k is those positive integers with k*n <= 600 - n.
// - Prints one line per input integer with the computed signed 64-bit value.
//
// Notes on choices relative to the Specification:
// - The spec assumes inputs are valid integers (n > 0 for meaningful behavior).
//   This implementation expects valid integer tokens; on a malformed token it
//   stops processing further input (the spec declares behavior unspecified for
//   malformed tokens).
// - Computation uses wider temporaries (i128) to avoid accidental intermediate
//   overflow for the intended ranges, then prints as a signed 64-bit integer
//   (i64), as required by the spec.
//
// This program tries to remain faithful to the Specification while being safe
// and idiomatic Rust.

use std::io::{self, Read};

fn main() {
    // Read all stdin into a string buffer.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit silently (mirrors unspecified I/O errors).
        return;
    }

    // Iterate whitespace-separated tokens, parse each as i32 (C int).
    for token in input.split_whitespace() {
        // The Specification assumes scanf("%d", &n) succeeds for intended input.
        // If parsing fails, the Specification declares behavior unspecified. We
        // choose to stop processing further input in that case.
        let n_i32: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Perform computation using 64/128-bit intermediates and print as i64.
        // Convert to i64 for arithmetic (mirrors typical C promotion to wider type).
        let n = n_i32 as i64;

        // For intended inputs n > 0. For other n, behavior is unspecified per spec.
        // We implement the mathematically consistent interpretation: if there is no
        // positive k with k*n <= 600 - n, the result is 0.
        let result_i64 = if n > 0 {
            let bound = 600i64 - n;
            // If bound < n, then no k >= 1 satisfies k*n <= bound.
            if bound < n {
                0i64
            } else {
                // m = floor(bound / n)
                let m = bound / n; // m >= 1
                // sum_{k=1..m} k^2 = m*(m+1)*(2m+1)/6
                let m128 = m as i128;
                let sum_k2 = m128 * (m128 + 1) * (2 * m128 + 1) / 6;
                let n128 = n as i128;
                let res128 = n128 * n128 * n128 * sum_k2; // n^3 * sum_k2
                // Cast to i64 for printing as signed 64-bit value.
                res128 as i64
            }
        } else {
            // For n <= 0 the Specification states behavior is unspecified.
            // Choose to produce 0 for safety (n <= 0 is outside intended use).
            0i64
        };

        println!("{}", result_i64);
    }
}
