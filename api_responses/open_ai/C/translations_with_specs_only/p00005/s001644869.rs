// gcd_lcm.rs
//
// Translates the provided Specification into Rust.
// - Reads pairs of signed 32-bit integers from stdin (whitespace separated).
// - For each pair (a, b) prints "<gcd> <lcm>\n" where gcd is greatest common divisor
//   and lcm is computed as (a / gcd) * b.
// Notes on semantic choices to stay faithful to the Specification:
// - Inputs are parsed as i32 (matching C's %d intent).
// - The code assumes well-formed input containing complete pairs of integers.
//   If parsing fails or an incomplete trailing token is encountered, processing stops.
//   This follows the Specification's assumption that scanf successfully reads pairs;
//   behavior on malformed/incomplete input is not specified.
// - To emulate possible C-style wraparound on multiplication of 32-bit signed ints,
//   lcm multiplication uses wrapping multiplication (i32::wrapping_mul).
//   The Specification states the original C program did not detect overflow; here
//   we avoid panics by using wrapping semantics.
// - We avoid unsafe code and keep everything in safe, idiomatic Rust.

use std::io::{self, Read};

fn abs_wrapping(x: i32) -> i32 {
    // Use wrapping negation to avoid panic/UB on i32::MIN.
    if x < 0 { x.wrapping_neg() } else { x }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    a = abs_wrapping(a);
    b = abs_wrapping(b);
    // Euclidean algorithm
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn main() {
    // Read entire stdin to a string and split on whitespace to mimic scanf behavior.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit quietly (Specification: normal termination on EOF; behavior
        // on IO errors not specified). We simply return.
        return;
    }

    let mut tokens = input.split_whitespace();
    loop {
        // Attempt to read first integer of the pair
        let a_token = match tokens.next() {
            Some(t) => t,
            None => break, // EOF - no more tokens
        };
        // Attempt to parse first integer; if parsing fails, stop processing further.
        let a: i32 = match a_token.parse() {
            Ok(v) => v,
            Err(_) => break, // unspecified behavior in spec for non-integer tokens; stop.
        };

        // Attempt to read second integer of the pair
        let b_token = match tokens.next() {
            Some(t) => t,
            None => break, // Incomplete pair at EOF: spec treats this as undefined; choose to stop.
        };
        let b: i32 = match b_token.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        let g = gcd(a, b);
        // Compute lcm as (a / g) * b using wrapping multiplication to emulate possible C overflow.
        // Division here uses normal i32 division; for intended positive inputs this is safe.
        let quotient = a / g; // follows spec: (a / gcd) * b
        let l = quotient.wrapping_mul(b);

        // Print as signed integers with a single space and a trailing newline.
        println!("{} {}", g, l);
    }
}
