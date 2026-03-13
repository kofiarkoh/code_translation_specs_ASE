// average.rs
//
// Translates the provided C-style specification into Rust.
// Behavior notes (to match the specification as closely as practical in Rust):
// - Inputs are read from stdin as whitespace-separated tokens.
// - The first token is parsed as a C-like `int` (emulated as i32).
// - The next n tokens are parsed as i32 and summed into an i32 accumulator.
// - Summation uses wrapping arithmetic to emulate typical two's-complement
//   behavior found on common C platforms (avoids Rust overflow panics).
//   The specification states signed overflow in C is undefined; many platforms
//   wrap, so wrapping_add is a reasonable faithful emulation.
// - Parsing errors or missing tokens are not handled (calls to unwrap will panic),
//   matching the specification that scanf failures are not checked.
// - If n == 0, integer division by zero will panic (matching the spec's undefined behavior).
// - If n < 0, no integers are read and sum stays 0; then 0 / n is computed (0 for nonzero n).
//
// The program prints a single line with the computed integer average (sum / n)
// using truncation toward zero (Rust's integer division semantics for i32 match C's).

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string.
    let mut input = String::new();
    // Any I/O error is not specially handled (will propagate as panic on unwrap).
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");

    // Tokenize by whitespace like scanf("%d").
    let mut tokens = input.split_whitespace();

    // Parse first token as n (C int -> i32).
    // Per specification, failures are not handled; mimic by unwrap (panic on error).
    let n: i32 = tokens
        .next()
        .expect("missing n")
        .parse()
        .expect("failed to parse n as int");

    // Sum using i32 with wrapping to emulate typical two's-complement behavior.
    let mut sum: i32 = 0;

    if n > 0 {
        // Read exactly n integers.
        // Parse each token as i32; unwrap on error to match unspecified behavior on parse failure.
        for _ in 0..(n as usize) {
            let v: i32 = tokens
                .next()
                .expect("missing integer token")
                .parse()
                .expect("failed to parse integer");
            sum = sum.wrapping_add(v);
        }
    } else {
        // If n <= 0, specification indicates no validation:
        // - n == 0 will later cause division by zero (undefined in C) -> here will panic.
        // - n < 0: loop does not iterate, sum remains 0.
    }

    // Compute average using C-like integer division semantics (truncation toward zero).
    // Rust i32 division uses truncation toward zero, so this matches the specification.
    let avg: i32 = sum / n; // will panic on division by zero if n == 0

    // Print the result followed by a newline.
    println!("{}", avg);
}
