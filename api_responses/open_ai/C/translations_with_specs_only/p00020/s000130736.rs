// ascii_upperline.rs
//
// Translates the given Specification into Rust:
// - Reads one line from stdin (terminated by newline or EOF).
// - Removes the terminating newline if present.
// - Converts ASCII lowercase letters 'a'..'z' to 'A'..'Z' (other characters unchanged).
// - Writes the transformed string followed by a newline to stdout and exits with status 0.
//
// Notes on fidelity to the Specification:
// - No explicit error handling is performed for read errors (we ignore the Result from read_line),
//   mirroring the Specification's note that the original C program made no check on gets' return.
// - The Specification mentions a fixed 81-byte buffer and an input-length precondition (<= 80).
//   This implementation does not enforce a length limit; the Specification said behavior is only
//   guaranteed when input length is <= 80. We do not invent additional checks or behavior.
// - ASCII-only lowercase-to-uppercase conversion is used (is_ascii_lowercase/to_ascii_uppercase),
//   preserving non-ASCII characters unchanged (consistent with ASCII-compatible encodings).
use std::io;

fn main() {
    let mut input = String::new();

    // Read one line from stdin. Per the Specification, we make no explicit error handling:
    // ignore the Result/number of bytes read.
    let _ = io::stdin().read_line(&mut input);

    // Remove a single terminating newline character if present.
    if input.ends_with('\n') {
        input.pop();
    }

    // Convert ASCII lowercase letters to uppercase; leave all other characters unchanged.
    let transformed: String = input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c
            }
        })
        .collect();

    // Write the transformed string followed by a newline (like puts).
    println!("{}", transformed);

    // Exit with status 0 (implicit).
}
