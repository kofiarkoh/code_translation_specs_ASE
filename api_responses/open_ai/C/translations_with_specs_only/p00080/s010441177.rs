// cube_root_rs.rs
//
// Translates the given specification into a Rust command-line program.
// Behavior follows the specification exactly:
// - Reads whitespace-separated integers from stdin.
// - If the integer is -1, terminates with exit code 0.
// - Otherwise, for each integer q, computes a floating-point approximation x
//   satisfying |x^3 - q| < 0.00001 * q (for q > 0).
// - Prints each x on its own line using the conventional "%f" formatting
//   (six digits after the decimal point).
//
// Notes on translation choices to preserve specified semantics:
// - The Newton-Raphson update used is x <- (2*x + q / (x*x)) / 3,
//   which is algebraically equivalent to x - (x^3 - q) / (3*x^2).
// - The initial guess is x = q (as f64). This matches the specification's
//   comment that q = 0 yields a zero initial guess and leads to division by
//   zero in the update step; we intentionally preserve that behavior.
// - Parsing failures are not specially handled; parse errors will cause a
//   panic (matching the specification's "unspecified" behavior on malformed input).
// - No artificial iteration limits are added; negative q (except -1) will
//   cause the tolerance check to use a negative threshold and thus the loop
//   will not terminate, which matches the specified (unspecified) behavior.

use std::io::{self, Read};

fn main() {
    // Read all of stdin as a string and split on whitespace into tokens.
    // The specification assumes successful integer reads; here we use unwrap()
    // so that malformed input causes a panic (unspecified behavior per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    for token in input.split_whitespace() {
        // Parse token as a signed C int. We use i32 to represent "C int".
        let q: i32 = token.parse().unwrap();

        if q == -1 {
            // Sentinel: normal termination with exit code 0 (implicit).
            return;
        }

        // Convert to f64 for computation.
        let qf = q as f64;

        // Initial guess: x = q (as f64). This preserves the spec behavior
        // where q = 0 leads to a zero initial guess and potential division by zero.
        let mut x = qf;

        // Tolerance threshold: 0.00001 * q (may be negative for q < 0).
        let threshold = 0.00001_f64 * qf;

        // Iterate Newton-Raphson until the residual is strictly less than the threshold.
        // The loop condition is while residual >= threshold so that we exit when residual < threshold.
        // For q <= 0 (except -1) this may never be true (spec: behavior unspecified / non-terminating).
        loop {
            let residual = (x.powi(3) - qf).abs();
            if residual < threshold {
                break;
            }
            // Update: x = (2*x + q/(x*x)) / 3
            // This may perform division by zero for x == 0 (e.g., q == 0), producing Inf/NaN,
            // which mirrors the unspecified behavior described in the specification.
            x = (2.0 * x + qf / (x * x)) / 3.0;
        }

        // Print with six digits after the decimal point to emulate printf "%f".
        println!("{:.6}", x);
    }

    // If EOF reached without sentinel, program simply returns (exit code 0).
}
