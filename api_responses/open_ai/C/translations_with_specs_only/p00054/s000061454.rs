use std::io::{self, Read};

/// This program implements the behavior described in the Specification:
/// It reads repeated triples of C `int` (we use i32) from stdin: (a, b, n).
/// For each triple it computes the sum of the first n decimal digits of the
/// fractional part of a/b using repeated long division:
///   r = a % b (C-like remainder)
///   repeat n times:
///     r = r * 10
///     d = r / b
///     r = r - b * d
///     sum += d
/// and prints sum followed by newline.
///
/// Notes on semantic choices to remain faithful to the Specification:
/// - We use i32 to model C `int`.
/// - C signed overflow is undefined; to avoid Rust panics in debug builds and
///   to approximate common two's-complement behavior on typical platforms,
///   integer multiplications/subtractions that could overflow are performed
///   with wrapping arithmetic (wrapping_mul/wrapping_sub/wrapping_add).
///   This choice is documented in the spec as an approximation of typical C
///   behavior on real systems. Division by zero is left to panic (as the
///   spec states behavior is undefined for b == 0).
/// - Token parsing: we read whitespace-separated tokens. If EOF occurs while
///   reading tokens we'll stop normally. If a token cannot be parsed as an
///   integer we stop processing further triples (the original C program's
///   behavior on malformed input is unspecified).
fn main() {
    // Read all stdin into a string and iterate over whitespace-separated tokens.
    let mut input = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut input) {
        // If reading stdin fails for some reason, exit silently (no triples processed).
        // The Specification does not require special handling.
        eprintln!("Failed to read stdin: {}", err);
        return;
    }

    let mut tokens = input.split_whitespace();

    loop {
        // Attempt to read three tokens for a, b, n. If any token is missing, stop.
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

        // Parse tokens to i32. If parse fails, stop processing further triples.
        // (Original C program's behavior on malformed input is unspecified.)
        let a: i32 = match a_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let b: i32 = match b_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let n: i32 = match n_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // If n <= 0 -> print 0 (per Specification).
        if n <= 0 {
            println!("0");
            continue;
        }

        // Initial remainder r = a % b using Rust's i32 `%` which matches C's
        // remainder semantics (truncating division, remainder has sign of numerator).
        // Note: if b == 0, this will panic (Specification treats division by zero as undefined).
        let mut r: i32 = a % b;

        // Sum accumulator as i32 to model C `int`. Use wrapping_add to approximate
        // two's-complement overflow behavior similar to typical C implementations.
        let mut sum: i32 = 0;

        // n is positive here; convert to usize for iteration. i32::MAX fits in usize on modern 64-bit.
        let iterations = n as usize;
        for _ in 0..iterations {
            // Emulate C's r *= 10 using wrapping to avoid Rust overflow panics and
            // to approximate common platform behavior.
            r = r.wrapping_mul(10);

            // Compute digit d = r / b. If b == 0 this panics (left as undefined per spec).
            let d: i32 = r / b;

            // r -= b * d  (use wrapping to approximate C behavior on overflow).
            // Note: compute b * d with wrapping_mul first.
            let bd = b.wrapping_mul(d);
            r = r.wrapping_sub(bd);

            // sum += d (wrapping to approximate C behavior)
            sum = sum.wrapping_add(d);
        }

        println!("{}", sum);
    }
}
