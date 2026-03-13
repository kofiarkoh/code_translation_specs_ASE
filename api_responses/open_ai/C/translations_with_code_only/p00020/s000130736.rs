use std::io::{self, Read, Write};

fn main() {
    // This program mirrors the behavior of the given C program as closely as is
    // reasonable and safe in Rust:
    //
    // - The C program uses gets(char[81]) which reads a line from stdin (no
    //   bounds checking in C). To avoid unsound buffer overflow in Rust we
    //   emulate the intended buffer size of 81 bytes (80 characters + NUL).
    //   Therefore we read a line from stdin and then truncate to at most 80
    //   bytes (leaving space conceptually for the NUL terminator).
    //
    // - Conversion is done byte-wise: ASCII 'a'..'z' are converted to uppercase
    //   by subtracting 32 (as in the original code). We operate on raw bytes to
    //   preserve the C behavior (which works on chars / bytes), not Unicode
    //   scalar values.
    //
    // - If the input contains an internal NUL (byte 0), the original C loop
    //   would stop at that NUL and puts would print up to it. We mirror that:
    //   processing stops at the first NUL and only bytes before it are written.
    //
    // - The original used puts which prints the string and then a newline.
    //   We write the bytes up to the first NUL (or up to the truncated length)
    //   and then write a single newline.
    //
    // These choices preserve observable I/O behavior (stdin/stdout formatting
    // and newline handling) while avoiding undefined behavior from buffer
    // overflow.

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut buf: Vec<u8> = Vec::new();

    // Read until newline (the newline, if present, is included in buf).
    // On EOF with no bytes read, read_until returns Ok(0).
    match handle.read_until(b'\n', &mut buf) {
        Ok(0) => {
            // No input (EOF). The original C code would have undefined behavior
            // if gets returned NULL and the program continued; the most benign
            // observable effect is to produce no output. We exit with success.
            return;
        }
        Ok(_) => { /* continue */ }
        Err(e) => {
            // On I/O error, mimic program aborting by printing nothing and
            // exiting with non-zero. Using eprintln here is acceptable.
            eprintln!("I/O error: {}", e);
            std::process::exit(1);
        }
    }

    // If the last byte is a newline, remove it (gets does not store the '\n').
    if buf.last() == Some(&b'\n') {
        buf.pop();
    }

    // Emulate fixed-size buffer char ch[81]: allow at most 80 characters stored.
    if buf.len() > 80 {
        buf.truncate(80);
    }

    // Find first NUL byte; processing and output stop at that position.
    let nul_pos = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());

    // Convert ASCII lowercase letters to uppercase by subtracting 32.
    for b in &mut buf[..nul_pos] {
        if b'a' <= *b && *b <= b'z' {
            *b = *b - 32; // same as ch[i] -= 32 in the C code
        }
    }

    // Write processed bytes up to the first NUL, then write a newline (puts).
    let stdout = io::stdout();
    let mut out = stdout.lock();
    if let Err(e) = out.write_all(&buf[..nul_pos]) {
        eprintln!("I/O error writing output: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = out.write_all(b"\n") {
        eprintln!("I/O error writing newline: {}", e);
        std::process::exit(1);
    }

    // Match the C program's successful exit code.
    std::process::exit(0);
}
