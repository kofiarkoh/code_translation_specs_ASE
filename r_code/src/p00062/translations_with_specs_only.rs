// spec_translation.rs
//
// Translation of the provided Specification to Rust.
// Behavior notes (kept faithful to the Specification):
// - Reads whitespace-separated tokens from stdin until EOF.
// - For each token, interprets the first ten characters as ASCII digits '0'..'9'.
//   (The Specification assumes tokens are exactly 10 characters long and all digits.
//    This implementation also assumes that; violating that may cause a panic.)
// - Computes a single decimal digit by repeatedly replacing the sequence of digits
//   with adjacent sums modulo 10 until one digit remains.
// - Stores results in a fixed-size storage for up to 10000 tokens (array of length 10000).
//   If more than 10000 tokens are provided, the program will panic (Specification declares
//   behavior undefined in that case).
// - After EOF, prints each stored result in the same order, one per line.
// - On normal completion, the process exits with code 0.

use std::io::{self, Read};

fn main() {
    // Read all stdin into a single String.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // According to the Specification, normal operation assumes EOF is eventually reached.
        // Any I/O error here is not specified; treat it by exiting (panic would be visible).
        panic!("failed to read stdin");
    }

    // Fixed-size storage for up to 10000 results, per Specification.
    let mut results: [u8; 10000] = [0; 10000];
    let mut count: usize = 0;

    // Iterate whitespace-separated tokens.
    for token in input.split_whitespace() {
        // Enforce the storage bound: the Specification requires token_count <= 10000;
        // behavior is undefined otherwise. Here we reflect that by panicking if exceeded.
        if count >= results.len() {
            panic!("too many tokens (exceeds 10000)");
        }

        // Interpret the first ten characters as decimal digits '0'..'9'.
        // The Specification guarantees tokens are exactly 10 chars of '0'..'9'.
        // We use the token's raw bytes and index the first 10 bytes directly.
        let bytes = token.as_bytes();

        // If the token is shorter than 10 bytes, indexing below will panic,
        // which is consistent with the Specification's undefined behavior for invalid input.
        let mut digits: [u8; 10] = [0; 10];
        for i in 0..10 {
            // Assumes ASCII '0'..'9'. Subtraction yields 0..9 for valid inputs.
            digits[i] = bytes[i] - b'0';
        }

        // Repeated adjacent summation modulo 10 until one digit remains.
        let mut n = 10;
        while n > 1 {
            for i in 0..(n - 1) {
                // digits[i] and digits[i+1] are small (0..9), sum fits in u8.
                digits[i] = (digits[i] + digits[i + 1]) % 10;
            }
            n -= 1;
        }

        // Store the single-digit result.
        results[count] = digits[0];
        count += 1;
    }

    // After EOF, print all stored results in order, one per line.
    // Each result is in 0..9 (per Specification preconditions).
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    use std::io::Write;
    for i in 0..count {
        // Write the digit as a decimal integer followed by a newline.
        // Using write! to avoid extra allocations.
        writeln!(handle, "{}", results[i]).expect("failed to write to stdout");
    }
}
