// translated from the provided C program following the SPEC.
// Reads whitespace-separated integers from stdin until EOF.
// For each integer `n` (assumed positive per SPEC), computes:
//   sum_{k: k*n <= 600 - n} (k*n)^2  then multiplies that sum by n
// and prints the result as a signed 64-bit decimal followed by a newline.
//
// Notes on deviations / safety choices:
// - The SPEC documents that inputs must be positive and that behavior for
//   non-integer tokens or n <= 0 is unspecified in the original C program.
//   To avoid non-termination or panics in Rust, this implementation:
//     * Stops processing further tokens when a token cannot be parsed as an integer.
//       (The C code's behavior on malformed tokens is undefined.)
//     * If a parsed integer `n` is <= 0, we print `0` and continue. The original C
//       program would exhibit undefined / non-terminating behavior for such values.
//       This choice avoids infinite loops while preserving intended behavior for
//       valid inputs (n > 0).
use std::io::{self, Read};

fn main() {
    // Read entire stdin as text and iterate over whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit silently (no output), matching the idea
        // that program terminates with code 0 on normal termination. We avoid
        // explicit error messages to stdout.
        return;
    }

    for token in input.split_whitespace() {
        // Try to parse token as a 32-bit signed integer, matching scanf("%d", &n).
        let n: i32 = match token.parse() {
            Ok(v) => v,
            // On parse failure, stop processing further tokens.
            // The original C program assumes tokens are integers; malformed input
            // leads to unspecified behavior. We choose to stop here.
            Err(_) => break,
        };

        // Per SPEC, inputs should be positive (n > 0). The original C code would
        // exhibit undefined or non-terminating behavior for n <= 0. To keep this
        // Rust program safe and to avoid infinite loops or panics, treat n <= 0
        // as producing 0.
        if n <= 0 {
            println!("0");
            continue;
        }

        let mut sum: i64 = 0;
        let upper = 600 - n; // fits in i32

        // If upper < n, the summation range is empty and sum remains 0.
        if upper >= n {
            let mut i = n;
            while i <= upper {
                let ii = i as i64;
                sum += ii * ii;
                i = i.wrapping_add(n); // wrapping_add is safe here because n > 0 and bounds are small
            }
        }

        sum *= n as i64;
        println!("{}", sum);
    }
}
