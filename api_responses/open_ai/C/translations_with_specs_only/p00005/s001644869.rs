// gcd_lcm_pairs.rs
//
// Translation of the given Specification into Rust.
//
// Notes on faithful translation choices:
// - Inputs are treated as C 'int' values; we use i32 to represent them.
// - The original C program used unchecked signed integer arithmetic.
//   Signed overflow in C is undefined behavior, but the Spec notes the
//   program performs no overflow checks. To emulate typical two's-complement
//   wraparound behavior (and avoid Rust panics in debug builds), multiplications
//   are performed with wrapping arithmetic.
// - The Spec describes an algorithm that repeatedly extracts common integer
//   factors by testing divisors i = 2..b and dividing both a and b when both
//   divisible. We implement the same loop structure and behavior.
// - Input is whitespace-separated tokens parsed as integers. We only process
//   complete pairs (two parsed integers). If an incomplete trailing token
//   exists (e.g. single integer with no partner), we stop (narrow, well-defined
//   interpretation consistent with "each successful processing requires two ints").
//
// Observable behavior preserved:
// - For each input pair (a,b) we print one line: "<c> <v>\n" where c is the
//   product of extracted common factors and v is computed as final_a * final_b * c
//   (mathematically (a*b)/c if inputs are positive and no overflow).
//
// This program uses safe Rust only.

use std::io::{self, Read};

fn main() {
    // Read entire stdin as UTF-8 text of whitespace-separated tokens.
    let mut input = String::new();
    // On read error we exit (this matches the C program terminating on IO failure).
    io::stdin().read_to_string(&mut input).unwrap_or(0);

    let mut iter = input.split_whitespace();

    // Process tokens two-at-a-time. Only process complete pairs.
    while let Some(a_tok) = iter.next() {
        let b_tok = match iter.next() {
            Some(t) => t,
            None => break, // incomplete pair: stop processing
        };

        // Parse into i32 (C 'int' is assumed to fit into i32 for this translation).
        // The Specification assumes well-formed integer tokens representable as int.
        let mut a: i32 = match a_tok.parse() {
            Ok(v) => v,
            Err(_) => break, // unspecified in spec; choose to stop on parse error
        };
        let mut b: i32 = match b_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // c accumulates the extracted common integer factors.
        let mut c: i32 = 1;

        // The algorithm iterates i from 2 up to b (using the current value of b,
        // which may be reduced during the process). Use i32 for i to mirror C int.
        let mut i: i32 = 2;
        // Note: If b < 2 initially, the loop body is skipped (matches C for-loop behavior).
        while i <= b {
            // While both a and b are divisible by i, divide them and multiply c by i.
            // Use wrapping_mul to emulate two's-complement wraparound on overflow.
            while (a % i == 0) && (b % i == 0) {
                // Division and remainder semantics (trunc toward zero, remainder sign)
                // in Rust match the intended C behavior for typical inputs.
                a /= i;
                b /= i;
                c = c.wrapping_mul(i);
            }
            i = i.wrapping_add(1);
        }

        // Compute v as final_a * final_b * c. Use wrapping arithmetic to emulate
        // unchecked signed overflow behavior in the original C program.
        let v = a.wrapping_mul(b).wrapping_mul(c);

        // Print results in the required format: "c v\n"
        println!("{} {}", c, v);
    }

    // Exit with status 0 implicitly.
}
