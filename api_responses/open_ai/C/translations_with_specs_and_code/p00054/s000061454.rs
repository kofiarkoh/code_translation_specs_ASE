use std::convert::TryFrom;
use std::io::{self, Read};

/// This program implements the behavior specified in the provided SPEC:
/// For each triple of integers (a, b, n) read from stdin until EOF, compute
/// the sum of the first n decimal digits of the fractional part of a/b,
/// produced by repeated long division, and print the sum followed by a newline.
///
/// Notes on implementation choices:
/// - The SPEC defines the digit selection as d = floor((r*10)/b). To follow
///   the SPEC we implement mathematical floor division for signed integers.
///   This can differ from the original C code's behavior for negative values
///   (see comment below).
/// - We use i128 for intermediate arithmetic to reduce the chance of overflow
///   panics in Rust. The SPEC states C signed overflow is undefined; using a
///   larger integer type here is a safety choice that preserves the intended
///   behavior for typical inputs (non-negative a and positive b).
/// - If b == 0 we let the program panic (division by zero) rather than
///   silently handle it; that matches the SPEC's statement that b == 0 is
///   undefined behavior.
/// - If parsing of an integer token fails we stop processing further triples
///   (this mirrors the C program which only checks for EOF and otherwise has
///   unspecified behavior on malformed input).
///
/// Mismatch with the C source:
/// - The C code finds the digit by searching j from 9 down to 0 with the test
///   `if (b*j <= a) break;` and then uses the loop variable `j` after the loop.
///   For the typical intended inputs (a >= 0, b > 0) this yields the same
///   result as floor((a*10)/b). However, for negative remainders the C loop
///   can leave j == -1 and produce behavior that differs from mathematical
///   floor division. The SPEC specifies digit = floor((r*10)/b), so this
///   implementation follows the SPEC (mathematical floor) rather than the
///   C edge-case behavior for negatives.
///
/// I/O: whitespace-separated integers from stdin; for each triple print one
/// line containing the sum and a newline.
fn main() {
    // Read entire stdin
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        // If stdin cannot be read, exit (mirrors failure in reading input).
        eprintln!("error reading stdin: {}", e);
        return;
    }

    let mut tokens = input.split_whitespace();
    loop {
        // Attempt to read three tokens; if EOF (None) encountered, stop.
        let a_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let b_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let n_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        // Parse tokens as signed integers. If parse fails, stop processing further.
        let a = match a_tok.parse::<i128>() {
            Ok(v) => v,
            Err(_) => break,
        };
        let b = match b_tok.parse::<i128>() {
            Ok(v) => v,
            Err(_) => break,
        };
        let n = match n_tok.parse::<i128>() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Precondition: b != 0. The SPEC marks division by zero undefined.
        if b == 0 {
            // Let Rust panic on division by zero (approximate undefined behavior).
            panic!("division by zero (b == 0)");
        }

        // r0 = a mod b in the SPEC sense: r0 = a - floor_div(a, b) * b
        let mut r = a - floor_div(a, b) * b;
        let mut sum: i128 = 0;

        // If n <= 0, process zero digits and sum remains 0.
        if n > 0 {
            // Convert n to usize for iteration count. If n is too large to convert,
            // that's outside typical/spec inputs; panic to avoid undefined behavior.
            let count = match usize::try_from(n) {
                Ok(c) => c,
                Err(_) => panic!("n too large to iterate"),
            };

            for _ in 0..count {
                // Multiply remainder by 10 and select digit as floor((r*10)/b)
                r = r * 10;
                let d = floor_div(r, b);
                r = r - d * b;
                sum += d;
            }
        }

        println!("{}", sum);
    }
}

/// Mathematical floor division for signed integers: floor(a / b).
/// Rust's `a / b` performs truncation toward zero. For positive values the
/// results coincide. For general signed inputs we adjust when the remainder
/// has opposite sign to the divisor and is non-zero.
fn floor_div(a: i128, b: i128) -> i128 {
    let q = a / b; // trunc toward zero
    let r = a % b;
    if r != 0 && ((r > 0) != (b > 0)) {
        q - 1
    } else {
        q
    }
}
