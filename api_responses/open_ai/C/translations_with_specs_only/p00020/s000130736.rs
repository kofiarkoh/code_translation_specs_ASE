// uppercase_line.rs
//
// Translates the provided specification into Rust.
// Behavior preserved per spec:
// - Uses a fixed 81-byte buffer (so up to 80 characters of input, plus a NUL).
// - Reads bytes from stdin until newline or EOF, or until 80 bytes are read.
// - Does in-place conversion of ASCII 'a'..'z' to uppercase by subtracting 32.
// - Ensures a NUL byte at buffer[len] and writes the bytes up to that NUL to stdout,
//   followed by a single newline produced by the output routine.
// - Exits normally with status 0 on normal completion.
//
// Notes on interpretation choices:
// - The spec describes a fixed 81-byte buffer and that safe behavior requires ≤80
//   characters. This implementation enforces the 80-byte content limit to keep
//   a clear, defined mapping to the buffer described. If stdin provides more than
//   80 characters before a newline, the remaining characters are left unread (the
//   loop stops when buffer is full), which matches the narrow interpretation of
//   using an 81-byte buffer and stopping at capacity.
// - I/O errors are treated as unrecoverable here by using unwraps when writing,
//   consistent with the spec's assumption that reads succeed under expected use.

use std::io::{self, Read, Write};

fn main() {
    // Fixed buffer of 81 bytes as per specification.
    let mut buf: [u8; 81] = [0u8; 81];
    let mut len: usize = 0;

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // Read bytes one by one until newline, EOF, or buffer capacity (80 bytes) reached.
    loop {
        if len >= 80 {
            // Buffer full (we reserve buf[80] for the NUL terminator).
            break;
        }
        let mut byte = [0u8; 1];
        match handle.read(&mut byte) {
            Ok(0) => {
                // EOF
                break;
            }
            Ok(1) => {
                if byte[0] == b'\n' {
                    // Stop at newline (do not include the newline in the buffer).
                    break;
                } else {
                    buf[len] = byte[0];
                    len += 1;
                }
            }
            Ok(_) => {
                // Shouldn't happen when reading 1 byte, but treat as EOF-like.
                break;
            }
            Err(_) => {
                // The specification does not define behavior on read failure.
                // Here we choose to return early without output (narrow interpretation).
                return;
            }
        }
    }

    // NUL-terminate the buffer at position len (preserve the invariant).
    if len < buf.len() {
        buf[len] = 0;
    }

    // In-place ASCII conversion: 'a'..'z' -> uppercase by subtracting 32.
    for i in 0..len {
        let b = buf[i];
        if b >= b'a' && b <= b'z' {
            // Subtract 32 to convert to uppercase as per ASCII assumption.
            buf[i] = b - 32;
        }
    }

    // Write the transformed bytes (up to len) followed by a newline.
    let mut stdout = io::stdout();
    // Writes are expected to succeed under normal preconditions; unwraps reflect that.
    stdout.write_all(&buf[..len]).unwrap();
    stdout.write_all(b"\n").unwrap();
    // Normal completion: exit code 0 (implicit).
}
