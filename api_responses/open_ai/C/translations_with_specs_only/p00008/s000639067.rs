// translator.rs
// Translates the provided specification to Rust.
// Behavior:
//   - Read whitespace-separated decimal integers from stdin until EOF.
//   - For each successfully read integer n (parsed as a signed 32-bit int),
//     print the number of ordered 4-tuples (a,b,c,d) with digits 0..9 such that a+b+c+d == n,
//     followed by a newline.
// Notes on interpretation choices (kept minimal and faithful to spec):
//   - The spec assumes input tokens are valid decimal integers. If a token fails to parse
//     as an i32 (non-numeric or out-of-range for i32), this implementation stops processing
//     further tokens. The original C-spec described unspecified behavior for that case;
//     choosing to stop is the narrowest reasonable behavior that avoids inventing extra handling.
//
// Complexity: For each input integer, this runs 10,000 checks (4 nested loops over 0..=9).

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string and split on whitespace.
    // The spec assumes tokens are whitespace-separated integers.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // Following the spec's minimal error handling: if reading stdin fails, exit.
        return;
    }

    for token in input.split_whitespace() {
        // Parse token as a C-like signed int. We use i32 as the closest match.
        // If parsing fails (non-integer token or overflow), stop processing further tokens.
        let n: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Count ordered 4-tuples of digits 0..9 whose sum equals n.
        // Use brute-force enumeration as specified (exactly 10,000 checks).
        let mut count: u32 = 0;
        for a in 0i32..=9 {
            for b in 0i32..=9 {
                for c in 0i32..=9 {
                    for d in 0i32..=9 {
                        if a + b + c + d == n {
                            count += 1;
                        }
                    }
                }
            }
        }

        // Print the count followed by a newline.
        println!("{}", count);
    }
}
