use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string and split into whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    // Parse tokens as i32. Per the Specification, tokens are assumed to be valid integers.
    // If parsing fails, unwrap() will panic; this mirrors the Specification's assumption
    // that scanf succeeds and malformed input leads to undefined behavior.
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Iterate over tokens manually: each test case begins with an integer n followed by n integers.
    let mut idx: usize = 0;
    const LOWER_BOUND: i32 = -1_000_000_000; // The specified running-min initialization.

    while idx < tokens.len() {
        let n = tokens[idx];
        idx += 1;

        // Terminate normally when n == 0, per Specification.
        if n == 0 {
            break;
        }

        // The Specification states behavior is undefined for n < 0 or n > 5000.
        // We choose the narrowest reasonable interpretation: stop processing if
        // there aren't enough tokens remaining for this test case (mirrors undefined behavior).
        if n < 0 {
            // Undefined behavior in spec; cease processing to avoid panics.
            break;
        }

        let n_usize = n as usize;
        if idx + n_usize > tokens.len() {
            // Insufficient tokens: undefined per spec. Stop processing.
            break;
        }

        // Algorithm:
        // Use prefix-sum based approach with running_min initialized to LOWER_BOUND.
        // Use wrapping arithmetic to emulate typical C int wrap-around (no overflow checks).
        let mut running_prefix: i32 = 0;
        let mut running_min: i32 = LOWER_BOUND;
        let mut max_sum: i32 = LOWER_BOUND;

        for j in 0..n_usize {
            let x = tokens[idx + j];
            running_prefix = running_prefix.wrapping_add(x);
            let candidate = running_prefix.wrapping_sub(running_min);
            if candidate > max_sum {
                max_sum = candidate;
            }
            if running_prefix < running_min {
                running_min = running_prefix;
            }
        }

        idx += n_usize;

        // Print result: one line per test case.
        println!("{}", max_sum);
    }

    // On normal termination (reading n == 0) the process exits with code 0 implicitly.
}
