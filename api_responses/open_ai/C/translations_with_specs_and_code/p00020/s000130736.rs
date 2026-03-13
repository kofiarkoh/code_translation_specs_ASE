// translate_gets_upper.rs
//
// This program implements the behavior described in the provided specification:
// - Read one input line (terminated by newline or EOF), remove the terminating newline,
//   convert ASCII lowercase letters ('a'..'z') to uppercase ('A'..'Z'), write the
//   resulting string to stdout followed by a newline, and exit with status 0.
//
// Notes about differences from the original C:
// - The C program used gets(), which is unsafe and can overflow a fixed 81-byte buffer.
//   The specification treats "input length <= 80" as a precondition for defined behavior.
//   This Rust translation follows the specification: it assumes the input satisfies that
//   precondition. Unlike the C code, this Rust code does not use unsafe operations or
//   deliberately introduce undefined behavior on overly long input.
//
// Implementation details:
// - Uses std::io::stdin().read_line to read one line (up to newline or EOF).
// - Removes only the trailing '\n' if present (matching gets() semantics that does not
//   include the newline in the stored string).
// - Converts ASCII lowercase bytes to uppercase by subtracting 32 from their byte value.
//   Other bytes are left unchanged. This matches the ASCII-compatible assumption in the spec.
// - Prints the transformed string with a single trailing newline (equivalent to puts).

use std::io::{self, Read};

fn main() {
    // Read one line from stdin into a String. This will include the trailing '\n' if present.
    let mut input = String::new();
    // According to the spec, standard input is assumed to be available; unwrap() is
    // used here to reflect that precondition (any I/O error will cause a panic).
    io::stdin().read_line(&mut input).unwrap();

    // Remove a single trailing newline character if present (gets() does not store the newline).
    if input.ends_with('\n') {
        input.pop();
    }

    // Work on the raw bytes to perform ASCII-only lowercase -> uppercase mapping.
    let mut bytes = input.into_bytes();
    for b in &mut bytes {
        if b'a' <= *b && *b <= b'z' {
            *b = *b - 32; // convert to ASCII uppercase
        }
    }

    // Convert back to String; this should succeed because we started from valid UTF-8
    // and only modified ASCII bytes.
    let output = String::from_utf8(bytes).expect("internal UTF-8 conversion error");

    // Print the result followed by a newline (puts-like behavior).
    println!("{}", output);
}
