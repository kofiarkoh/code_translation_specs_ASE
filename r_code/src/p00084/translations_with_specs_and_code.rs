use std::io::{self, Write};

fn main() {
    // Read one line from stdin into a String.
    // SPEC: the program reads at most 1024 characters into its buffer (buffer capacity 1025 incl. NUL).
    // We read the entire line from stdin, then truncate to 1024 bytes to respect the buffer limit.
    // Note: unlike C's fgets which may leave remaining input in stdin when the line is longer
    // than the buffer, Rust's read_line consumes the full line. We truncate the in-memory buffer
    // to 1024 bytes to preserve the program's subsequent behavior (removing final character, tokenizing).
    let mut s = String::new();
    let n = io::stdin().read_line(&mut s).unwrap_or(0);
    // SPEC precondition: stdin must provide data (fgets must succeed). If there is no input (EOF),
    // behavior is undefined. Here we choose to return quietly in that case.
    if n == 0 {
        return;
    }

    // Convert to bytes and enforce 1024-byte capacity as per SPEC.
    let mut buf = s.into_bytes();
    if buf.len() > 1024 {
        buf.truncate(1024);
    }

    // Remove the final character of the read buffer (intended to remove trailing newline).
    // This mirrors the C code's `line[i-2]=0` behavior which unconditionally drops the last char.
    if !buf.is_empty() {
        buf.pop();
    }

    // Tokenize by delimiters: space (0x20), comma ',', and period '.'
    // and collect tokens whose byte-length is between 3 and 6 inclusive.
    let is_delim = |b: u8| b == b' ' || b == b',' || b == b'.';
    let mut tokens: Vec<&[u8]> = Vec::new();

    let mut i = 0usize;
    while i < buf.len() {
        // skip delimiters
        if is_delim(buf[i]) {
            i += 1;
            continue;
        }
        let start = i;
        while i < buf.len() && !is_delim(buf[i]) {
            i += 1;
        }
        let tok = &buf[start..i];
        let len = tok.len();
        if (3..=6).contains(&len) {
            tokens.push(tok);
        }
    }

    // Print selected tokens separated by single spaces (no leading/trailing spaces), then newline.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut first = true;
    for tok in tokens {
        if !first {
            handle.write_all(b" ").unwrap();
        } else {
            first = false;
        }
        handle.write_all(tok).unwrap();
    }
    handle.write_all(b"\n").unwrap();
}
