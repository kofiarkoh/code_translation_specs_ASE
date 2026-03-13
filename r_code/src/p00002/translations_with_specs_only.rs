// sum_len.rs
//
// Translation of the provided Specification into Rust.
// Behavior:
//  - Repeatedly read pairs of integers from stdin (whitespace-separated).
//  - For each pair (a, b), compute s = a + b (wrapping on overflow to emulate typical C two's-complement behavior).
//  - Print the number of characters in the decimal representation of s (including '-' if negative), followed by a newline.
// Notes about choices made to stay safe in Rust while remaining faithful to the Spec:
//  - The Spec describes scanf("%d %d") use. Here we accept the same textual input shape: signed decimal integers separated by whitespace.
//  - If parsing a token fails, or EOF is reached before a complete pair, the program stops processing further input.
//    The original Spec describes undefined behavior for malformed input; stopping is the narrow, safe interpretation.
//  - Integer addition uses wrapping_add to avoid Rust debug-build panics on overflow and to approximate typical C two's-complement wraparound.
//  - The Spec mentions a 16-byte buffer in the original C; Rust String is used safely instead.
//  - Program exits with code 0 on normal termination.

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails for some reason, exit normally (specifies exit code 0).
        return;
    }

    // Iterate over whitespace-separated tokens.
    let mut tokens = input.split_whitespace();

    loop {
        // Attempt to read first integer token.
        let a_tok = match tokens.next() {
            Some(t) => t,
            None => break, // EOF before a pair -> terminate normally
        };
        // Attempt to parse as i32 (C 'int' reasonably maps to 32-bit here per Spec).
        let a: i32 = match a_tok.parse() {
            Ok(v) => v,
            // On parse failure, Spec allows undefined behavior; choose to stop processing further input.
            Err(_) => break,
        };

        // Attempt to read second integer token.
        let b_tok = match tokens.next() {
            Some(t) => t,
            None => break, // EOF before completing the pair -> terminate normally
        };
        let b: i32 = match b_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Compute sum. Use wrapping_add to emulate typical C two's-complement wraparound (avoids panic on overflow).
        let s = a.wrapping_add(b);

        // Format decimal representation and measure length (number of characters, excluding any NUL).
        let s_str = s.to_string();
        let len = s_str.len();

        // Print length followed by newline.
        println!("{}", len);
    }

    // Exit with code 0 (implicit).
}
