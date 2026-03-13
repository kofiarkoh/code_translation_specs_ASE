// Translated from the provided C program.
// Notes about preserving semantics:
// - The C code does `x = q/2;` where q is an int. That uses integer division in C.
//   We preserve that by performing integer division in Rust (`q / 2`) and then
//   casting to f64.
// - The Newton iteration and the stopping condition use floating-point comparisons
//   exactly as in C. For negative `q` the RHS 0.00001*q is negative, so the
//   comparison is always false and the loop never breaks (matching the C behavior).
// - For q == 0 the iteration performs a 0/(3*x*x) with x==0, producing NaN in
//   IEEE754; Rust f64 semantics match this and will produce NaN and further
//   iterations behave similarly to C. We do not try to "fix" this undefined/edge case.
// - Input handling: the original C code used scanf without checking its return
//   value. To avoid undefined behavior in Rust we read whitespace-separated tokens
//   from stdin and stop when tokens are exhausted. This is the usual faithful
//   translation strategy for such programs.
// - Output formatting: C's printf("%lf\n", x) prints a double with 6 digits after
//   the decimal (same as "%f"). We use "{:.6}" for finite numbers. For NaN and
//   infinities, many C runtimes print "nan"/"inf" in lowercase; Rust prints "NaN"/"inf".
//   To better match typical C output we map NaN/inf to lowercase strings.
//
// The program reads integers until it encounters -1 (then it breaks) and for each
// integer prints the computed x followed by a newline.

use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse whitespace-separated tokens as integers.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    for token in input.split_whitespace() {
        // If a token fails to parse as i32, stop processing further tokens.
        let q: i32 = match token.parse() {
            Ok(n) => n,
            Err(_) => break,
        };

        if q == -1 {
            break;
        }

        let qf = q as f64;

        // Preserve C semantics: integer division q/2, then convert to f64.
        let mut x = (q / 2) as f64;

        // Newton iteration:
        // while(!(fabs(x*x*x - q) < 0.00001*q)){
        //   x = x - (x*x*x-q)/(3*x*x);
        // }
        while !((x * x * x - qf).abs() < 0.00001 * qf) {
            x = x - (x * x * x - qf) / (3.0 * x * x);
        }

        // Print with 6 fractional digits like printf("%lf\n", x).
        // For NaN/inf, produce lowercase "nan"/"inf" to more closely match typical C output.
        let out = if x.is_nan() {
            "nan".to_string()
        } else if x.is_infinite() {
            if x.is_sign_negative() {
                "-inf".to_string()
            } else {
                "inf".to_string()
            }
        } else {
            format!("{:.6}", x)
        };

        println!("{}", out);
    }
}
