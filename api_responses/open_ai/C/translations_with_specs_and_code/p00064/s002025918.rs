// sum_digits.rs
//
// Translation of the provided C program per the specification:
// Reads all whitespace-separated tokens from stdin, extracts every contiguous
// sequence of ASCII digits '0'..'9' within each token, interprets each such
// sequence as a non-negative base-10 integer, and computes the sum of these
// integers. Prints the sum followed by a newline and exits with status 0.
//
// Notes about choices made to preserve semantics and safety:
// - The C program used a fixed 81-byte buffer for tokens; the SPEC states
//   tokens should be at most 80 characters but behavior is unspecified if
//   longer. This Rust implementation accepts arbitrarily long tokens safely.
//   This is a deliberate safety improvement (no buffer overflow).
// - The C program used signed `int` and had undefined behavior on integer
//   overflow. To avoid panics in Rust debug builds while preserving a simple
//   semantics, this implementation uses i32 and performs arithmetic with
//   wrapping (modular) operations. This yields well-defined wrap-around on
//   overflow rather than undefined behavior.
// - Digit classification uses ASCII digits only (ch.is_ascii_digit()),
//   matching the SPEC's '0'..'9' requirement rather than locale-dependent
//   behavior of C's isdigit().

use std::io::{self, Read};

fn main() {
    // Read all input from stdin
    let mut input = String::new();
    // If reading fails, we let the program panic (mirrors that the SPEC
    // assumes stdin is available). unwrap() keeps code concise.
    io::stdin().read_to_string(&mut input).unwrap();

    // Use 32-bit signed integers to mirror typical C `int`.
    // Use wrapping arithmetic to avoid debug-build panics on overflow.
    let mut sum: i32 = 0;

    // Split on whitespace to obtain tokens (whitespace delimitation
    // matches the SPEC behavior).
    for token in input.split_whitespace() {
        let mut num: i32 = 0;
        let mut in_run = false;

        for ch in token.chars() {
            if ch.is_ascii_digit() {
                let digit = (ch as u8 - b'0') as i32;
                if !in_run {
                    // Start a new digit run
                    in_run = true;
                    num = 0;
                }
                // Accumulate digit with wrapping to avoid panics on overflow.
                num = num.wrapping_mul(10).wrapping_add(digit);
            } else {
                if in_run {
                    // End of a digit run: add accumulated number to sum.
                    sum = sum.wrapping_add(num);
                    in_run = false;
                }
                // Non-digit characters are separators; continue scanning.
            }
        }

        // If token ended while in a digit run, add the last number.
        if in_run {
            sum = sum.wrapping_add(num);
        }
    }

    // Print the sum followed by a newline, as required.
    println!("{}", sum);
}
