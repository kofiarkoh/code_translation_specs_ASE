// sensor_buckets.rs
//
// Translates the provided specification into Rust.
// Behavior notes (from specification):
// - Read whitespace-separated numeric tokens from stdin as f64, in pairs: (right, left).
// - For each pair, classify each value into exactly one of four buckets using chained comparisons:
//     if v >= 1.1 => bucket 0
//     else if v >= 0.6 => bucket 1
//     else if v >= 0.2 => bucket 2
//     else => bucket 3
//   This choice preserves the typical C idiom (chained if/else) so every processed value
//   increments exactly one bucket; NaN comparisons are false, and therefore NaN falls to the final else,
//   which matches common implementation behavior.
// - For well-formed input (complete pairs of doubles) counts reflect processed values.
// - If tokens are malformed or an incomplete final pair occurs, the program stops processing further input
//   (a narrow reasonable interpretation of the unspecified/undefined behavior from the original spec).
//
// The program prints four lines to stdout, each "rn[i] ln[i]", for i = 0..3.

use std::io::{self, Read};

fn classify_and_inc(v: f64, buckets: &mut [u64; 4]) {
    // Chained comparisons so every value increments exactly one bucket.
    if v >= 1.1 {
        buckets[0] += 1;
    } else if v >= 0.6 {
        buckets[1] += 1;
    } else if v >= 0.2 {
        buckets[2] += 1;
    } else {
        // This branch covers v < 0.2 and also values for which all comparisons are false (e.g., NaN).
        buckets[3] += 1;
    }
}

fn main() {
    // Read all stdin to a string.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, per spec there is no special error output. Exit silently.
        return;
    }

    let mut rn: [u64; 4] = [0; 4];
    let mut ln: [u64; 4] = [0; 4];

    let mut iter = input.split_whitespace();
    loop {
        // Expect a pair: right, left. If tokens are missing or fail to parse, stop processing.
        let t_right = match iter.next() {
            Some(t) => t,
            None => break, // EOF or no more tokens
        };
        let t_left = match iter.next() {
            Some(t) => t,
            None => break, // incomplete final pair: stop (narrow reasonable interpretation)
        };

        // Parse tokens as f64. If parse fails, stop processing further input.
        let r: f64 = match t_right.parse() {
            Ok(v) => v,
            Err(_) => break, // malformed token: stop
        };
        let l: f64 = match t_left.parse() {
            Ok(v) => v,
            Err(_) => break, // malformed token: stop
        };

        classify_and_inc(r, &mut rn);
        classify_and_inc(l, &mut ln);
    }

    // Print four lines: "rn[i] ln[i]" for i = 0..3
    for i in 0..4 {
        println!("{} {}", rn[i], ln[i]);
    }
}
