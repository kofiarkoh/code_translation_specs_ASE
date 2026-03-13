// translation.rs
//
// Translated from the provided C program according to the specification.
// Behavior:
//  - Reads whitespace-separated tokens from stdin interpreted as pairs of f64:
//      first = "right", second = "left".
//  - For each successfully parsed pair, increments one of four buckets for each channel
//    according to these ranges:
//      bucket 0: value >= 1.1
//      bucket 1: 0.6 <= value < 1.1
//      bucket 2: 0.2 <= value < 0.6
//      bucket 3: value < 0.2   (also receives NaN because comparisons with NaN are false)
//  - After input is exhausted, prints four lines:
//      "rn[0] ln[0]"
//      "rn[1] ln[1]"
//      "rn[2] ln[2]"
//      "rn[3] ln[3]"
//  - Exits with status 0.
//
// Notes about differences from the original C:
//  - The original C used scanf("%lf %lf", &right, &left) and did not properly check
//    the number of conversions; behavior on malformed or partial input was undefined.
//    This Rust translation follows the specification's defined behavior for well-formed
//    input (complete pairs of doubles). On a parse error or an incomplete final pair
//    we stop processing further input (which is a reasonable, well-defined choice).
//    This is consistent with the spec's emphasis on well-formed inputs.
//  - Classification logic preserves the original ordering of comparisons so that NaN
//    values (for which >= comparisons are false) end up in bucket 3, matching the C
//    comparison semantics.
//
// Implementation details:
//  - Uses safe, idiomatic Rust only (no unsafe).
//  - Reads entire stdin into a String and iterates over whitespace-separated tokens.
//  - For each token pair, attempts to parse both as f64; if either fails, stop processing.

use std::io::{self, Read};

fn main() {
    // Counters for right (rn) and left (ln) channels, initialized to zero.
    let mut rn: [i64; 4] = [0, 0, 0, 0];
    let mut ln: [i64; 4] = [0, 0, 0, 0];

    // Read all input from stdin.
    let mut input = String::new();
    // If reading fails, treat as no input and proceed to printing zeros.
    let _ = io::stdin().read_to_string(&mut input);

    let mut tokens = input.split_whitespace();

    loop {
        // Attempt to get the next two tokens. If either is missing, stop processing.
        let r_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let l_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        // Parse tokens as f64. On parse error, stop processing further input.
        let right = match r_tok.parse::<f64>() {
            Ok(v) => v,
            Err(_) => break,
        };
        let left = match l_tok.parse::<f64>() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Classify right value into rn buckets.
        if right >= 1.1 {
            rn[0] += 1;
        } else if right >= 0.6 {
            rn[1] += 1;
        } else if right >= 0.2 {
            rn[2] += 1;
        } else {
            rn[3] += 1;
        }

        // Classify left value into ln buckets.
        if left >= 1.1 {
            ln[0] += 1;
        } else if left >= 0.6 {
            ln[1] += 1;
        } else if left >= 0.2 {
            ln[2] += 1;
        } else {
            ln[3] += 1;
        }
    }

    // Print results: four lines, each "rn[i] ln[i]\n" for i = 0..3.
    for i in 0..4 {
        println!("{} {}", rn[i], ln[i]);
    }
}
