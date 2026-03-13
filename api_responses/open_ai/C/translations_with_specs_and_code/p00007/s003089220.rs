// translation of the provided C program according to the SPEC:
// - start with a = 100000
// - repeat n times: a := floor(1.05 * a); then if not a multiple of 1000, round up to next multiple of 1000
// - read n from stdin (expected to be a valid integer); print final a followed by a newline
//
// Notes on implementation choices:
// - To compute floor(1.05 * a) exactly for positive integers we use integer arithmetic: (105 * a) / 100.
// - The multiplication uses i64 temporarily to avoid Rust debug-build overflow panics for large values;
//   the SPEC states signed-int overflow is undefined for the original C program, and it assumes inputs
//   that avoid overflow. Using i64 for the intermediate is a practical choice to preserve the intended semantics
//   for all reasonable inputs while keeping the outer type as a 32-bit integer like the C source did.
// - Parsing uses unwrap() to mirror the C program's lack of error handling for scanf failure (i.e. undefined behavior).

use std::io::{self, Read};

fn main() {
    // Read stdin, take first token, parse as i32. If parsing fails, unwind (mirrors undefined behavior of C's scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n_token = input.split_whitespace().next().unwrap();
    let n: i32 = n_token.parse().unwrap();

    let mut a: i32 = 100_000;

    if n > 0 {
        // iterate n times; for negative n the loop does not run (matches the C semantics)
        for _ in 0..n {
            // compute floor(1.05 * a) as (105 * a) / 100
            let tmp = (105i64 * a as i64) / 100;
            let mut next = tmp as i32;

            // round up to next multiple of 1000 if not already a multiple
            if next % 1000 != 0 {
                next = ((next / 1000) + 1) * 1000;
            }

            a = next;
        }
    }

    // print the final value followed by a newline, and exit (Rust main returns with exit code 0)
    println!("{}", a);
}
