// cube_root.rs
//
// Translated from the provided C program following the SPEC.
// - Reads whitespace-separated integers from stdin.
// - For each integer q:
//     - if q == -1: exit(0).
//     - otherwise: compute a floating-point approximation x of cube root of q
//       using the same iterative formula and stopping criterion as the C code:
//         initial x = q/2  (integer division, then cast to f64)
//         iterate while |x^3 - q| >= 0.00001 * q:
//           x = x - (x^3 - q) / (3 * x^2)
//       and print x with six digits after the decimal point (matching printf "%f").
// Notes:
// - This translation preserves the C semantics per SPEC, including the integer
//   division used for the initial guess and the same stopping criterion.
// - The program assumes valid integer tokens on stdin. Parsing failures will
//   cause a panic (to mirror the original program's unspecified behavior on
//   invalid input).
// - For q <= 0 (except sentinel -1) behavior is not defined by the SPEC and
//   is therefore preserved as in the C program (may diverge, not guaranteed to terminate).

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string and split on whitespace.
    // The SPEC states the program assumes successful integer reads; here we
    // unwrap parses to mimic that assumption (invalid input will panic).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for token in input.split_whitespace() {
        let q: i32 = token.parse().unwrap(); // mirrors C's assumption of valid integers
        if q == -1 {
            // Normal termination via sentinel; exit with status 0 by returning from main.
            return;
        }

        // Preserve C behavior: initial x = q/2 uses integer division before cast to double.
        let mut x: f64 = (q / 2) as f64;
        let q_f = q as f64;
        // Iteratively apply Newton-Raphson update until the residual is strictly less
        // than 0.00001 * q, matching the C condition: !(fabs(x*x*x - q) < 0.00001*q)
        while (x * x * x - q_f).abs() >= 0.00001 * q_f {
            // Note: this may produce non-finite values or division-by-zero behavior for
            // certain inputs (e.g., q == 0 leading to x == 0), which matches the C program's semantics.
            x = x - (x * x * x - q_f) / (3.0 * x * x);
        }

        // Print with six digits after the decimal point to match printf "%f" default precision.
        println!("{:.6}", x);
    }
}
