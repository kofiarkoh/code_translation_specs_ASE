// translate_spec.rs
//
// This program implements the provided specification:
// - Reads exactly five integer tokens from standard input (interpreted as C `int`).
// - Orders them in non-increasing (descending) order.
// - Prints the five integers to stdout separated by single spaces, with a trailing space
//   after the fifth integer. No newline is guaranteed.
// - On normal execution (valid input), the process exits with status 0.
//
// Notes on faithful translation choices:
// - We choose i32 to represent C `int` (typical on common platforms).
// - The original C program did not check scanf return values; to mirror that lack of
//   validation we use `unwrap()` on token extraction and parsing. This will cause a
//   panic (non-zero exit) on malformed/insufficient input, which corresponds to the
//   "undefined" behavior described in the specification rather than silently handling it.
// - Sorting is implemented with an O(n^2) selection-style algorithm for clarity and to
//   match the small fixed n = 5 constraint.
// - Output formatting exactly uses single spaces between numbers and a trailing space,
//   with no guaranteed newline.

use std::io::{self, Read};

fn main() {
    // Read all stdin into a string.
    let mut input = String::new();
    // Use unwrap to propagate I/O errors as panics; spec assumes normal environment.
    io::stdin().read_to_string(&mut input).unwrap();

    // Tokenize on whitespace. According to the spec, inputs are whitespace- or
    // format-separated integer tokens.
    let mut toks = input.split_whitespace();

    // Parse exactly five integers. We intentionally do not handle parse errors or
    // missing tokens gracefully to reflect the specification's lack of scanf checks.
    // Using unwrap() will panic if a token is missing or not a valid i32.
    let mut vals: [i32; 5] = [
        toks.next().unwrap().parse::<i32>().unwrap(),
        toks.next().unwrap().parse::<i32>().unwrap(),
        toks.next().unwrap().parse::<i32>().unwrap(),
        toks.next().unwrap().parse::<i32>().unwrap(),
        toks.next().unwrap().parse::<i32>().unwrap(),
    ];

    // Sort in non-increasing (descending) order.
    // Use a simple selection-like algorithm (O(n^2)), appropriate for n = 5.
    for i in 0..5 {
        // find index of maximum in vals[i..]
        let mut max_idx = i;
        for j in (i + 1)..5 {
            if vals[j] > vals[max_idx] {
                max_idx = j;
            }
        }
        if max_idx != i {
            vals.swap(i, max_idx);
        }
    }

    // Print the five integers separated by single spaces, with a trailing space.
    // No newline is printed (none is guaranteed by the spec).
    for v in &vals {
        print!("{} ", v);
    }

    // Normal termination: implicit exit with status 0.
}
