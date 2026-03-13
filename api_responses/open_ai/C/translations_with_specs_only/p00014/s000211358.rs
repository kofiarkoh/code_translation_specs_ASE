// translation.rs
//
// Translates the given specification into Rust.
// - Reads whitespace-separated decimal integers from stdin (like scanf("%d")).
// - For each integer `n` computes:
//     result = n * sum_{i = n, n*2, ... <= 600 - n} (i^2)
//   and prints the result as a signed 64-bit decimal on its own line.
//
// Notes on faithful semantics:
// - Tokens are parsed as a C `int` (i32). The spec assumes well-formed inputs, so parsing errors are treated as fatal here (via unwrap/expect).
// - The per-input loop uses i and step of type i32 and follows the condition `i <= 600 - n`. For n == 0 or negative n this may not terminate per the spec; we preserve that behavior.
// - Arithmetic on the 64-bit accumulator uses wrapping operations to emulate unchecked/unchecked-overflow behavior on a 64-bit signed accumulator (long long).
// - No input validation beyond parsing is performed, matching the specification.

use std::io::{self, Read};

fn main() {
    // Read entire stdin as text. The spec expects whitespace-separated decimal integers until EOF.
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");

    // Iterate over whitespace-separated tokens. The spec assumes tokens are valid decimal integers.
    for token in input.split_whitespace() {
        // Parse token as a C int (i32). We expect well-formed input per the spec.
        let n: i32 = token
            .parse()
            .expect("invalid integer token (spec assumes well-formed integers)");

        // Accumulator and arithmetic are performed in i64 (signed 64-bit), using wrapping ops to emulate unchecked overflow.
        let mut sum: i64 = 0;
        let bound: i32 = 600 - n;

        // Loop variable and step are i32 to preserve behavior for edge cases like n == 0 (which leads to a non-terminating loop).
        let mut i: i32 = n;

        // Note: This loop intentionally does not guard against n == 0 or negative n, per specification.
        while i <= bound {
            let ii = i as i64;
            // Use wrapping_mul/wrapping_add to emulate unchecked 64-bit arithmetic (like a C long long without overflow checks).
            sum = sum.wrapping_add(ii.wrapping_mul(ii));
            i = i.wrapping_add(n);
        }

        // Final result is n * sum, as a signed 64-bit value; use wrapping_mul to match unchecked semantics.
        let result: i64 = sum.wrapping_mul(n as i64);

        // Print one line per input integer.
        println!("{}", result);
    }
}
