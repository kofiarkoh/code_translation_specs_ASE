// gcd_lcm.rs
//
// Translated from the provided C program according to the SPEC.
// Note: The original C code used `while (scanf("%d%d", &a, &b) != EOF)` which
// treats the scanf return value in a non-standard way (it doesn't check that
// two items were matched). The SPEC requires that two integers are available
// per iteration. This Rust translation follows the SPEC: it reads whitespace-
// separated tokens from stdin and processes them as pairs of integers; parsing
// stops if a token cannot be parsed as an integer or if an incomplete pair is
// encountered. This behavior preserves the intent of the SPEC (process complete
// pairs of ints, one output line per pair). 
//
// Implementation notes:
// - Uses safe, idiomatic Rust only.
// - Uses the Euclidean algorithm for GCD (works efficiently).
// - LCM is computed as (a / gcd) * b using 64-bit intermediates to avoid
//   accidental intermediate overflow; the final printed values are cast to
//   32-bit signed integers (i32) as per SPEC. The SPEC assumes inputs and
//   results fit in the platform's signed int; no overflow checking is done.

use std::io::{self, Read};

fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    // Normalize: work with absolute values so gcd is non-negative.
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    a
}

fn main() {
    // Read all input from stdin.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we can't read stdin, just exit (matches C program which would likely
        // fail on scanf). No output is produced.
        return;
    }

    let mut iter = input.split_whitespace();

    loop {
        // Get first token of the pair.
        let t1 = match iter.next() {
            Some(s) => s,
            None => break, // EOF / no more tokens
        };
        // Parse first integer; if parsing fails, stop processing further (matches SPEC assumptions).
        let a_i32: i32 = match t1.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Get second token of the pair.
        let t2 = match iter.next() {
            Some(s) => s,
            None => break, // Incomplete pair at EOF: stop (SPEC marks this as undefined; we stop).
        };
        let b_i32: i32 = match t2.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Convert to i64 for safe intermediate arithmetic.
        let a64 = a_i32 as i64;
        let b64 = b_i32 as i64;

        // Compute gcd and lcm according to SPEC for positive inputs.
        let g = gcd_i64(a64, b64);

        // lcm = (a / gcd) * b. Use absolute values so lcm is non-negative for positive inputs.
        let l = if g == 0 {
            // If both a and b are zero, define lcm as 0 (pragmatic choice;
            // SPEC does not require behavior for zeros).
            0i64
        } else {
            (a64 / g) * b64
        };

        // According to SPEC, outputs are platform signed ints. Cast to i32 for printing.
        // The SPEC assumes values fit in the signed int range; no overflow checks are performed.
        let g_out: i32 = g as i32;
        let l_out: i32 = l as i32;

        // Print "<gcd> <lcm>\n"
        println!("{} {}", g_out, l_out);
    }
}
