// translation of the provided C program to Rust following the SPEC.
// Notes:
// - Per SPEC, inputs are whitespace-separated tokens; each token's first
//   ten characters are ASCII digits '0'..'9'.
// - The SPEC requires that the number of tokens be <= 10000. The original C
//   code used a fixed-size array and would overflow if more tokens were given.
//   To preserve the SPEC invariant, this Rust version stops processing further
//   tokens once 10000 results have been collected (avoids unbounded growth).
//   This is a deliberate divergence from the undefined/unsafe behavior the C
//   program would have exhibited on >10000 tokens.
// - This implementation assumes the preconditions hold (each token has at least
//   ten ASCII digit characters). Violating that may panic (matching the SPEC's
//   "undefined" stance for invalid inputs).

use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");

    const MAX_RESULTS: usize = 10000;
    let mut results: Vec<u8> = Vec::with_capacity(MAX_RESULTS);

    for token in input.split_whitespace() {
        if results.len() >= MAX_RESULTS {
            // Enforce SPEC invariant: do not store more than 10000 results.
            break;
        }

        let bytes = token.as_bytes();
        // Per SPEC precondition, the token must have at least 10 chars which are '0'..'9'.
        // Indexing below will panic if this is not true (acceptable per SPEC's undefined behavior).
        let mut seq: [u8; 10] = [
            bytes[0] - b'0',
            bytes[1] - b'0',
            bytes[2] - b'0',
            bytes[3] - b'0',
            bytes[4] - b'0',
            bytes[5] - b'0',
            bytes[6] - b'0',
            bytes[7] - b'0',
            bytes[8] - b'0',
            bytes[9] - b'0',
        ];

        // Repeated adjacent summation modulo 10: reduce length 10 -> 1
        for j in 0..9 {
            for k in 0..(9 - j) {
                seq[k] = (seq[k] + seq[k + 1]) % 10;
            }
        }

        results.push(seq[0]);
    }

    // After EOF, print all stored results, one per line, in input order.
    let mut out = String::with_capacity(results.len() * 2);
    for &d in &results {
        // d is 0..9; append digit and newline
        out.push((b'0' + d) as char);
        out.push('\n');
    }
    print!("{}", out);
}
