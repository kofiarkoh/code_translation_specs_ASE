// segments_slope_compare.rs
//
// Translates the provided specification into Rust.
// Behavior notes (per specification):
// - Reads an integer n, then for each of n test cases reads 8 doubles:
//   x1 y1 x2 y2 x3 y3 x4 y4
// - Computes slopes as (y2 - y1) / (x2 - x1) and (y4 - y3) / (x4 - x3)
//   using f64 division and compares them with exact equality.
// - Prints "YES" if slopes are exactly equal, otherwise "NO", one line per test case.
// - This implementation assumes well-formed input; parsing uses unwrap() to mirror the
//   specification's lack of input-validation (missing/invalid tokens will panic, which
//   corresponds to unspecified behavior per the spec).
// - Floating-point division by zero follows IEEE-754 semantics in Rust (Inf/NaN).

use std::io::{self, Read};

fn main() {
    // Read entire stdin as a string and split on whitespace.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    let mut toks = input.split_whitespace();

    // Parse the number of test cases. Per spec, assume this token exists and is a valid integer.
    let n_token = toks.next().unwrap();
    let n: i64 = n_token.parse().unwrap();

    if n <= 0 {
        // Per spec: if n <= 0, no test cases are processed. Normal termination with exit code 0.
        return;
    }

    // Convert to usize for looping; n >= 1 here so conversion is safe for typical ranges.
    // If n is absurdly large and doesn't fit in usize, this will wrap on some platforms;
    // but the spec does not define bounds, and we choose a direct conversion.
    let count = n as usize;

    for _ in 0..count {
        // For each test case, read 8 doubles. Per spec, tokens are expected to be present and valid.
        let x1: f64 = toks.next().unwrap().parse().unwrap();
        let y1: f64 = toks.next().unwrap().parse().unwrap();
        let x2: f64 = toks.next().unwrap().parse().unwrap();
        let y2: f64 = toks.next().unwrap().parse().unwrap();
        let x3: f64 = toks.next().unwrap().parse().unwrap();
        let y3: f64 = toks.next().unwrap().parse().unwrap();
        let x4: f64 = toks.next().unwrap().parse().unwrap();
        let y4: f64 = toks.next().unwrap().parse().unwrap();

        // Compute slopes as exact f64 divisions. This may produce Inf or NaN per IEEE-754.
        let slope1 = (y2 - y1) / (x2 - x1);
        let slope2 = (y4 - y3) / (x4 - x3);

        if slope1 == slope2 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
