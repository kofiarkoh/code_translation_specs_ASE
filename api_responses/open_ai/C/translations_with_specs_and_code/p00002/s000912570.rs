// translate_sum_len.rs
//
// Translation of the provided C program according to the SPEC.
// - Reads pairs of integers from stdin.
// - For each pair (a, b) computes s = a + b, obtains the decimal string
//   representation of s (like C's "%d"), and prints the number of characters
//   in that representation followed by a newline.
// Notes on choices made to match the SPEC:
// - We treat input tokens as C 'int' values -> use i32.
// - The SPEC's precondition requires that a + b does not overflow a C int.
//   We do not perform overflow handling here; we simply use Rust's i32 + operator.
//   If the precondition is violated, behavior is unspecified (same as the SPEC).
// - We stop processing when fewer than two whitespace-separated tokens remain,
//   which aligns with the SPEC assumption that EOF occurs at record boundaries.
// - The program uses safe, idiomatic Rust and streams tokens from stdin.
// - Output formatting exactly matches the SPEC: one decimal integer per line,
//   then '\n', no extra whitespace.
//
// This program returns a non-zero exit code on I/O errors (propagated via `?`).
// Under normal termination (EOF at a record boundary) it exits with code 0.

use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Read all stdin into a String. The SPEC assumes well-formed input;
    // this is a simple and safe way to obtain whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut tokens = input.split_whitespace();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    // Process tokens in pairs. If fewer than two tokens remain, stop.
    loop {
        let a_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let b_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        // Parse tokens as i32 (C 'int' mapping). If parsing fails, stop processing.
        // The SPEC leaves malformed input behavior unspecified; here we choose to stop.
        let a: i32 = match a_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let b: i32 = match b_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Compute sum. SPEC requires that this does not overflow a C int.
        // We do not add explicit overflow handling here.
        let s = a + b;

        // Convert to decimal string (includes '-' for negatives), count chars, print.
        let s_str = s.to_string();
        let len = s_str.len();
        writeln!(out, "{}", len)?;
    }

    Ok(())
}
