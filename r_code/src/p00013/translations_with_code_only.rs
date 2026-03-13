// main.rs
// Translated from the provided C program.
// Notes on translation choices (preserve semantics as in the C source):
// - Uses fixed-size arrays of 100 i32 elements for `car` and `out` to match the C code.
// - Uses usize for indices `i` and `k`. Decrementing `i` when it's zero will panic in Rust
//   (index underflow / out-of-bounds), which corresponds to the original C program's undefined
//   behavior when inputs are not well-formed. We keep that behavior rather than silently protecting it.
// - Input is parsed as whitespace-separated tokens and each token is parsed as an i32.
//   This matches the common case of scanf("%d", ...) with well-formed integer input.
//   Non-integer tokens are ignored (scanf would behave differently on malformed input, but the
//   usual contest-style inputs are integers only).
// - Output formatting preserves the C program: each integer on its own line, no extra whitespace.
//
// This is a direct, idiomatic Rust translation intended to behave the same as the C program for
// well-formed inputs.

use std::io::{self, Read};

fn main() {
    // Read entire stdin to a string (looping until EOF like scanf in the C program).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut car = [0i32; 100];
    let mut out = [0i32; 100];
    let mut i: usize = 0;
    let mut k: usize = 0;

    for token in input.split_whitespace() {
        // Parse token as i32. If parsing fails we skip the token.
        // (scanf would behave differently on malformed input; typical inputs are integers.)
        if let Ok(n) = token.parse::<i32>() {
            if n == 0 {
                // Mirror the C code: decrement i, then read car[i] into out[k], then increment k.
                // If i == 0 this will panic (underflow index) — similar to UB in C when input is malformed.
                i -= 1;
                out[k] = car[i];
                k += 1;
            } else {
                // Push behavior: store at car[i] then increment i.
                car[i] = n;
                i += 1;
            }
        }
    }

    // Print outputs exactly as in the C program: each on its own line.
    for idx in 0..k {
        println!("{}", out[idx]);
    }
}
