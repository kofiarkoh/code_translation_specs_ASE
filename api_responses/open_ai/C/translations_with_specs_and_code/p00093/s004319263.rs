// leap_years.rs
//
// Translated from the provided C program according to the specification.
// Behavior:
//  - Repeatedly reads pairs of integers (a b) from stdin.
//  - For each pair except the terminating (0 0):
//      - Prints each leap year y in [a, b], one per line, in ascending order.
//      - If none, prints a single line "NA".
//      - Prints an additional blank line after each pair's output.
//  - Stops processing when the pair (0 0) is read.
// Notes:
//  - This program assumes well-formed input as specified (two integers per query,
//    and a terminating "0 0" pair). Parsing errors will cause the program to panic,
//    which mirrors the unspecified behavior of the original C program on malformed input.

use std::io::{self, Read};

fn main() {
    // Read entire stdin. The C program used scanf without checking return values;
    // the spec assumes well-formed input. We follow that assumption and use
    // unwrap() on parse results.
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");

    let mut tokens = input.split_whitespace();
    loop {
        // Attempt to read two tokens; if fewer than two remain, stop.
        let a_token = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let b_token = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        // Parse into i32. The specification treats inputs as C `int` (commonly 32-bit).
        // On malformed input this will panic (unspecified in spec).
        let a: i32 = a_token.parse().unwrap();
        let b: i32 = b_token.parse().unwrap();

        // Termination condition: do not print anything for (0, 0).
        if a == 0 && b == 0 {
            break;
        }

        let mut found = false;

        // Iterate inclusive from a to b. If a > b, the loop is skipped (empty range).
        if a <= b {
            for y in a..=b {
                // Leap-year predicate from spec:
                // divisible by 400, or divisible by 4 and not by 100.
                if y % 400 == 0 || (y % 4 == 0 && y % 100 != 0) {
                    println!("{}", y);
                    found = true;
                }
            }
        }

        if !found {
            println!("NA");
        }

        // Print a blank line after each processed pair (except terminating pair).
        println!();
    }
}
