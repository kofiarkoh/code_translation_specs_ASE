// translator.rs
//
// Translates the given specification into a Rust program.
// Behavior:
//  - Repeatedly read pairs of integers from stdin (whitespace-separated).
//  - For each pair (a, b), compute s = a + b (emulate C int wraparound).
//  - Print the number of characters in the decimal representation of s
//    (including '-' for negative numbers), followed by a newline.
//
// Notes on fidelity to the Specification:
//  - C `int` is modeled as a 32-bit signed integer (i32). The spec treats
//    signed overflow as undefined; to produce a deterministic, C-like
//    two's-complement behavior, we use wrapping_add to emulate wraparound.
//    This matches common platform behavior where signed overflow wraps.
//  - Input tokens are parsed with Rust's decimal integer parser which accepts
//    optional '+' or '-' and digits, similar to C's %d conversion.
//  - The program assumes well-formed input (pairs of integers). If parsing
//    fails or an incomplete pair is encountered, the program stops silently
//    (the specification leaves such behavior unspecified).
//  - Output is one line per processed pair containing only the length and '\n'.
//
// Implementation choices:
//  - For simplicity and clarity we read stdin to a String and iterate
//    whitespace-separated tokens. This is straightforward and preserves
//    the required observable I/O behavior.
//  - All I/O errors (reads/writes) use unwrap only for brevity; in the
//    context of the spec these are not explicitly handled.

use std::io::{self, Write};

fn main() {
    // Read entire stdin into a String. The spec assumes stdin is readable and
    // contains a sequence of integer pairs; this is a reasonable, simple approach.
    let mut input = String::new();
    // If reading fails, treat as empty input and exit normally.
    let _ = io::stdin().read_to_string(&mut input);

    let mut out = io::BufWriter::new(io::stdout());

    let mut tokens = input.split_whitespace();

    loop {
        // Attempt to get next two tokens. If not available, stop (EOF / incomplete pair).
        let ta = match tokens.next() {
            Some(s) => s,
            None => break,
        };
        let tb = match tokens.next() {
            Some(s) => s,
            None => break,
        };

        // Parse tokens as i32 (modeling C `int`).
        // If parsing fails, stop silently (behavior unspecified by spec).
        let a = match ta.parse::<i32>() {
            Ok(v) => v,
            Err(_) => break,
        };
        let b = match tb.parse::<i32>() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Emulate C signed integer behavior by using wrapping_add (two's-complement wrap).
        let s = a.wrapping_add(b);

        // Convert to decimal string as C's %d would produce (no leading zeros,
        // '-' for negatives). Measure the length (number of characters).
        let dec = s.to_string();
        let len = dec.len(); // ASCII digits and optional '-' -> byte length equals char count

        // Write the length followed by newline.
        // Unwrap on write errors per spec (I/O errors are not explicitly handled).
        writeln!(out, "{}", len).unwrap();
    }

    // Flush output; ignore errors per spec.
    let _ = out.flush();
}
