// sum_len.rs
//
// Translated from the given C program according to the provided specification.
// Behavior notes (matching the SPEC):
// - Reads whitespace-separated tokens from stdin, consumes them in pairs.
// - For each pair of integers (i32), computes their sum and prints the number
//   of characters in the decimal representation of the sum (including '-' if negative).
// - On EOF before any pair is processed: prints nothing and exits successfully.
// - If a token cannot be parsed as an integer or if a pair is incomplete, the
//   program intentionally panics (undefined behavior per spec).
//
// Implementation choices:
// - Uses i32 as the program integer type to match typical C `int` width.
// - Uses safe Rust only. Parsing errors and incomplete pairs use `.expect()` / `panic!`
//   to model the SPEC's "undefined" behavior (i.e., may crash).
// - Reads entire stdin to a String and iterates tokens; this preserves exact
//   whitespace/line-oriented output (each result is a number followed by '\n').

use std::io::{self, Read};

fn main() {
    // Read all stdin into a string. If reading fails, exit (matching that other I/O
    // errors are not fully specified by the original program).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // Other I/O errors: behavior not fully specified; choose to exit successfully.
        return;
    }

    let mut tokens = input.split_whitespace();

    // Consume tokens in pairs. If a second token for a pair is missing, this is
    // treated as undefined behavior per SPEC; here we panic to reflect that.
    loop {
        let a_tok = match tokens.next() {
            Some(t) => t,
            None => break, // EOF before next pair: normal termination
        };

        let b_tok = match tokens.next() {
            Some(t) => t,
            None => {
                // Incomplete pair: SPEC states behavior is undefined.
                panic!("incomplete integer pair on input");
            }
        };

        // Parse tokens as i32. If parsing fails, SPEC says behavior is undefined;
        // propagate that by panicking.
        let a: i32 = a_tok.parse().expect("invalid integer token");
        let b: i32 = b_tok.parse().expect("invalid integer token");

        // Compute sum. SPEC requires the sum be representable in the program's integer type;
        // if it is not, behavior is undefined. We perform normal addition.
        let sum = a + b;

        // Convert to decimal string and print its character length (includes '-' if negative).
        let repr = sum.to_string();
        println!("{}", repr.len());
    }
}
