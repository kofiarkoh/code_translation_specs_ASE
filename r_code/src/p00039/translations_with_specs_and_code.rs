// roman.rs
//
// Translated from the provided C program according to the SPEC.
// Behavior:
// - Read whitespace-delimited tokens from stdin until EOF.
// - For each token, map characters to values: I=1, V=5, X=10, L=50, C=100, D=500, any other char=1000.
// - Values are stored in a fixed-size array of length 101 which is reused across tokens and
//   NOT cleared between tokens (this preserves the observable "leftover data" behavior described
//   in the SPEC).
// - Apply the usual Roman subtractive/additive rule left-to-right. Print one integer per token,
//   followed by a newline.
//
// Notes on differences from the C implementation:
// - The original C used a fixed char buffer of size 101 and scanf("%s", str) which is undefined
//   behavior for tokens longer than 100 bytes. To keep the Rust program safe we only process the
//   first 100 characters of a token; this avoids unsafe memory behavior while preserving the core
//   semantics. This choice is documented and intentional.
//
// No `unsafe` is used.

use std::io::{self, Read};

fn main() {
    // Read all stdin into a string and iterate tokens split by whitespace.
    // This replicates scanf("%s") tokenization semantics.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit (no output). This keeps behavior simple and safe.
        return;
    }

    // Fixed-size array of 101 integers, initialized to zero once and reused across tokens.
    // This preserves the SPEC's noted behavior where leftover values from previous tokens
    // may affect later computations.
    let mut a: [i32; 101] = [0; 101];

    for token in input.split_whitespace() {
        // s: accumulated sum for this token
        let mut s: i32 = 0;

        // Fill the values array for positions corresponding to the token's characters.
        // We limit to the first 100 characters to avoid buffer overflow semantics of C.
        // The array slot immediately after the last character (a[l]) is intentionally left
        // as-is (could be a leftover from a previous token).
        let mut l: usize = 0;
        for (i, ch) in token.chars().enumerate() {
            if i >= 100 {
                // Stop filling further positions to simulate the fixed 100-char buffer.
                break;
            }
            a[i] = match ch {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                _ => 1000,
            };
            l = i + 1;
        }

        // Apply the subtractive/additive rule left-to-right.
        let mut i = 0usize;
        while i < l {
            // Compare current with the next slot. Note that a[i+1] may contain a leftover
            // from a previous token if i+1 >= l; this reproduces the original program's behavior.
            if a[i] < a[i + 1] {
                s += a[i + 1] - a[i];
                i += 2; // skip the next position as it's been consumed
            } else {
                s += a[i];
                i += 1;
            }
        }

        // Print the computed integer followed by a newline.
        println!("{}", s);
    }
}
