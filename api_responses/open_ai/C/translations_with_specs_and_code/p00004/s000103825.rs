// linear_system.rs
//
// Translated from the provided C program according to the SPEC.
// Notes on mismatch with C source:
// - The SPEC defines y = (a*f - c*d)/D and x = (c*e - b*f)/D with D = a*e - b*d.
//   The original C used algebraically equivalent expressions with negated
//   numerator and denominator for y. The translation follows the SPEC explicitly.
// - The C code used `float`; this translation uses `f64` for the computations
//   (printing still uses three decimal places). Using f64 is safe here because
//   the SPEC guarantees integer intermediates do not overflow the platform int,
//   so values fit exactly into f64 integer range for these operations.
//
// Behavior notes (as per SPEC):
// - Reads zero or more records of six whitespace-separated integers from stdin.
// - For each complete record computes the unique solution (x,y) of the 2x2 system
//   using the algebraic formulae (Cramer's-rule form).
// - Applies normalization: if value in (-0.0005, 0] set it to 0 before printing.
// - Prints one line per record: "<x_formatted> <y_formatted>\n", both with 3 decimals.
// - On EOF or on any parse failure/incomplete record, processing stops and program exits.
//
// Implementation choices:
// - Input is tokenized with split_whitespace; non-integer tokens or incomplete final
//   records stop processing (matching the SPEC's "not handled" behavior for such cases).
// - Computations performed in f64 for robustness and to match printf-style floating output.

use std::io::{self, Read};

fn main() {
    // Read all of stdin into a string
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we cannot read stdin, just exit (matches normal program termination behavior).
        return;
    }

    // Tokenize by whitespace
    let mut iter = input.split_whitespace();

    loop {
        // Attempt to read six integers; stop if any token is missing or fails to parse.
        let a_tok = match iter.next() {
            Some(t) => t,
            None => break,
        };
        let b_tok = match iter.next() {
            Some(t) => t,
            None => break,
        };
        let c_tok = match iter.next() {
            Some(t) => t,
            None => break,
        };
        let d_tok = match iter.next() {
            Some(t) => t,
            None => break,
        };
        let e_tok = match iter.next() {
            Some(t) => t,
            None => break,
        };
        let f_tok = match iter.next() {
            Some(t) => t,
            None => break,
        };

        // Parse tokens as i32. If parsing fails, stop processing (SPEC: non-integer tokens not handled).
        let a_i: i32 = match a_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let b_i: i32 = match b_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let c_i: i32 = match c_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let d_i: i32 = match d_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let e_i: i32 = match e_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let f_i: i32 = match f_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Convert to f64 for floating-point computation.
        let a = a_i as f64;
        let b = b_i as f64;
        let c = c_i as f64;
        let d = d_i as f64;
        let e = e_i as f64;
        let f = f_i as f64;

        // Determinant
        let dnm = a * e - b * d; // D

        // Per SPEC: x = (c*e - b*f) / D ; y = (a*f - c*d) / D
        let mut x = (c * e - b * f) / dnm;
        let mut y = (a * f - c * d) / dnm;

        // Tiny negative-to-zero normalization: if (-0.0005, 0] -> 0
        if x > -0.0005 && x <= 0.0 {
            x = 0.0;
        }
        if y > -0.0005 && y <= 0.0 {
            y = 0.0;
        }

        // Print with exactly three digits after the decimal, separated by a single space.
        // Each line ends with a newline.
        println!("{:.3} {:.3}", x, y);
    }
}
