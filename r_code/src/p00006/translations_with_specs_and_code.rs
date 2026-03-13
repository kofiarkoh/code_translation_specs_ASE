// reverse_token.rs
//
// Translated from the provided C program according to the SPEC.
// Notes about a mismatch with the C code:
// - The original C code copied the reversed bytes into strsub but did not write a terminating NUL
//   into strsub before calling printf("%s", strsub). That is potentially undefined behavior.
//   The SPEC requires that, when preconditions hold (token length L in 1..20), the program
//   writes exactly L bytes (the input token reversed) to stdout. Therefore this Rust translation
//   follows the SPEC and writes exactly L bytes (no terminating NUL, no newline).
//
// The program:
// - Reads bytes from stdin, extracts the first whitespace-delimited token (as bytes).
// - Assumes token length L satisfies 1 <= L <= 20 under normal usage.
// - Writes the token's bytes in reverse order to stdout as a contiguous sequence (no newline).
//
// For invalid/violating inputs (no token or length > 20) the program exits with non-zero status.
// This is safer than reproducing undefined behavior from the original C.

use std::io::{self, Read, Write};
use std::process;

fn main() {
    // Read all bytes from stdin
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        // Reading failed; treat as an error (SPEC: scanf failure is undefined).
        process::exit(1);
    }

    // Find the first whitespace-delimited token (as bytes).
    // Skip leading ASCII whitespace
    let mut start = 0usize;
    while start < input.len() && input[start].is_ascii_whitespace() {
        start += 1;
    }
    if start >= input.len() {
        // No token found (similar to scanf failing / EOF): undefined per SPEC.
        // We exit non-zero.
        process::exit(1);
    }
    // Find end of token
    let mut end = start;
    while end < input.len() && !input[end].is_ascii_whitespace() {
        end += 1;
    }

    let token = &input[start..end];
    let len = token.len();

    // SPEC precondition: 1 <= L <= 20. Enforce here; otherwise treat as error.
    if len == 0 || len > 20 {
        // Out-of-spec input: behavior is undefined in original program.
        // Here we return non-zero to avoid undefined behavior.
        process::exit(1);
    }

    // Two fixed-size buffers of capacity 21 bytes each (per SPEC).
    let mut buf: [u8; 21] = [0u8; 21];
    let mut buf_rev: [u8; 21] = [0u8; 21];

    // Copy token into buf[0..len]
    buf[..len].copy_from_slice(token);

    // Reverse into buf_rev[0..len]
    for i in 0..len {
        buf_rev[i] = buf[len - 1 - i];
    }

    // Write exactly L bytes to stdout (the reversed token), no newline.
    if io::stdout().write_all(&buf_rev[..len]).is_err() {
        // Writing failed
        process::exit(1);
    }

    // Successful termination
    process::exit(0);
}
