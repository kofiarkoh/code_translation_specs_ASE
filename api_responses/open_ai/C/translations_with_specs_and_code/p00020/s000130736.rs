// uppercase_line.rs
//
// Translation of the provided C program to Rust according to the specification.
// Behavior:
// - Reads a single line from stdin (stopping at a newline or EOF).
// - Converts ASCII 'a'..'z' to uppercase by subtracting 32 from the byte value.
// - Writes the transformed line to stdout followed by a newline.
// Notes:
// - This implementation assumes ASCII-compatible input as in the spec.
// - The original C program used a fixed 81-byte buffer and exhibited undefined
//   behavior for lines > 80 bytes. The spec treats such cases as undefined;
//   this Rust program does not attempt to emulate buffer overflow and processes
//   the entire input line (the spec's precondition remains: input <= 80 chars).
// - No unsafe code is used.

use std::io;

fn main() {
    // Read a line from stdin into a String.
    // read_line includes the trailing newline (if present).
    let mut s = String::new();
    let n = match io::stdin().read_line(&mut s) {
        Ok(n) => n,
        // On I/O error, behavior is unspecified by the spec; exit quietly.
        Err(_) => return,
    };

    // If EOF occurred before any data was read, behave like the spec's undefined
    // condition: do nothing and exit.
    if n == 0 {
        return;
    }

    // Remove trailing newline (and optional CR from CRLF).
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    } else if s.ends_with('\r') {
        // Handle bare CR just in case.
        s.pop();
    }

    // Work with the UTF-8 bytes. The spec assumes ASCII; s originated from a
    // valid UTF-8 String, so from_utf8 below should succeed.
    let mut bytes = s.into_bytes();
    for b in &mut bytes {
        if *b >= b'a' && *b <= b'z' {
            // Uppercase by subtracting 32, as specified.
            *b = *b - 32;
        }
    }

    // Convert back to String (should succeed) and print with a trailing newline.
    if let Ok(out) = String::from_utf8(bytes) {
        // println! appends a newline, matching the C program's puts().
        println!("{}", out);
    } else {
        // This branch is extremely unlikely because we started with a valid String.
        // If it does occur, exit without printing (spec allows undefined behaviour
        // for unexpected encodings).
        return;
    }
}
