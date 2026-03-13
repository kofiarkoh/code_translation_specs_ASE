// sum_len.rs
//
// Translated from the provided C program and SPEC.
//
// Behavior:
// - Repeatedly reads whitespace-separated integer tokens from stdin.
// - For each pair of integers (a, b) read successfully, computes s = a + b
//   and prints the number of characters in the decimal representation of s,
//   followed by a newline.
// - Stops when EOF is encountered before a complete pair is available.
//
// Notes about translation choices and divergence from the original C semantics:
// - The SPEC documents that the original C program exhibits undefined behavior
//   if scanf returns a value other than EOF but less than 2 (e.g., malformed
//   tokens). In C that could lead to using uninitialized variables.
//   In safe Rust we cannot reproduce UB; instead this implementation stops
//   processing when a token fails to parse as an integer (i.e., it treats
//   malformed input as end-of-processing).
// - The SPEC also notes that integer overflow of a + b is undefined in C.
//   To avoid Rust debug-mode panics on overflow while keeping a deterministic
//   behavior similar to typical C wrap-around, this implementation uses
//   wrapping_add. This is a pragmatic choice to avoid panics while keeping
//   behavior defined (the SPEC does not require checking for overflow).
// - The original C used a fixed 16-byte buffer for the formatted sum and
//   would have undefined behavior if the decimal representation exceeded 15
//   characters. Here we compute the full decimal representation and print its
//   length. The SPEC requires that inputs avoid such long results; we do not
//   enforce or emulate a buffer overflow.
//
// This program aims to satisfy the SPEC's core functional behavior and I/O
// formatting: each output line is a single base-10 integer (the length) and a
// newline. The process exits with code 0 on normal termination.

use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Read entire stdin into a string. This matches the SPEC assumption that
    // input is whitespace-separated text on stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut stdout = io::stdout();
    let mut tokens = input.split_whitespace();

    loop {
        // Attempt to get two tokens. If EOF (no further tokens) occurs before a
        // complete pair, terminate normally (exit code 0).
        let a_tok = match tokens.next() {
            Some(s) => s,
            None => break,
        };
        let b_tok = match tokens.next() {
            Some(s) => s,
            None => break,
        };

        // Try to parse both tokens as 32-bit signed integers (C 'int' is used
        // by the SPEC; we use i32 to match typical 32-bit int behavior).
        // If parsing fails for either token, the SPEC documents undefined
        // behavior for the original program; here we stop processing further
        // input rather than invoking UB.
        let a = match a_tok.parse::<i32>() {
            Ok(v) => v,
            Err(_) => break,
        };
        let b = match b_tok.parse::<i32>() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Perform addition. To avoid Rust debug-build overflow panics and to
        // have deterministic behavior, use wrapping_add. The SPEC permits
        // integer overflow to be undefined; using wrapping_add is a defined,
        // deterministic choice that avoids crashes.
        let sum = a.wrapping_add(b);

        // Format sum as decimal string and measure its length (number of
        // characters, including '-' for negative numbers). This corresponds to
        // the C program's sprintf into a 16-byte buffer and strlen.
        let s = sum.to_string();
        let len = s.len();

        // Print the length followed by a newline. Match exact formatting.
        writeln!(stdout, "{}", len)?;
    }

    Ok(())
}
