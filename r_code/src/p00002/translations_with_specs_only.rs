// sum_len.rs
//
// Reads pairs of integers from standard input until EOF. For each pair,
// computes their arithmetic sum and writes the number of characters in the
// decimal representation of the sum (including a leading '-' if negative),
// followed by a newline.
//
// Implementation notes (kept minimal to stay faithful to the Specification):
// - The program uses i32 as the "program integer type". This choice ensures
//   that decimal representations for any valid i32 value fit within the
//   fixed-size formatting buffer capacity stated in the Specification (16
//   bytes). Max i32 decimal length is 11 (e.g. "-2147483648") which is < 16.
// - Token parse errors or incomplete pairs cause an immediate panic via
//   unwrap(), matching the Specification's "undefined behavior" for malformed
//   input or incomplete pairs (the narrowest reasonable interpretation).
// - If the decimal representation would somehow exceed the 16-byte buffer
//   (which should not occur for i32), the program asserts and panics.
// - For well-formed input (pairs of i32s until EOF), the program prints one
//   line per pair containing a non-negative integer (>= 1) equal to the
//   length of the decimal representation of the sum.

use std::io::{self, Read, Write};

fn main() {
    // Read entire stdin into a string.
    let mut input = String::new();
    // If reading fails for other I/O reasons, let the program propagate the error by panicking:
    // the Specification does not require handling other I/O errors.
    io::stdin().read_to_string(&mut input).unwrap();

    let mut stdout = io::stdout();
    let mut tokens = input.split_whitespace();

    // Process pairs sequentially.
    while let Some(a_tok) = tokens.next() {
        // If there's no second token, unwrap() will panic, modeling undefined behavior
        // for incomplete pairs as per the Specification.
        let b_tok = tokens.next().unwrap();

        // Parse tokens as i32. On parse failure, unwrap() panics (undefined behavior).
        let a: i32 = a_tok.parse().unwrap();
        let b: i32 = b_tok.parse().unwrap();

        let sum = a + b; // For well-formed inputs, sum is representable in i32 per preconditions.

        let sum_str = sum.to_string();

        // The Specification mentions an internal fixed-size formatting buffer of 16 bytes.
        // For the chosen i32 type, any decimal representation is guaranteed to be < 16 bytes.
        // To stay faithful, assert this precondition; violation causes a panic (undefined).
        assert!(
            sum_str.len() < 16,
            "decimal representation exceeds internal buffer capacity"
        );

        // The output is the length in characters (bytes for ASCII digits and '-').
        let len = sum_str.len();

        // Write the length followed by a newline. On write errors, unwrap() will panic.
        writeln!(stdout, "{}", len).unwrap();
    }

    // On EOF, simply return; process exits with success status.
}
