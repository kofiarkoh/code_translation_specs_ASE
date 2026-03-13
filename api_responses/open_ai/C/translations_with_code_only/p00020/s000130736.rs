use std::io::{self, Read, Write};

fn main() {
    // This program mirrors the behavior of the given C program:
    // - Reads a line from stdin (like gets), removing the trailing newline.
    // - Converts ASCII lowercase letters 'a'..'z' to uppercase by subtracting 32.
    // - Writes the resulting C-style string followed by a newline (like puts).
    //
    // Notes on semantics preservation and safe Rust choices:
    // - The original C used gets(ch) with ch declared as char ch[81].
    //   gets has no bounds checking and can overflow the buffer (undefined behavior).
    //   To avoid unsafe memory UB in Rust, we emulate the typical intended behavior:
    //   we copy at most 80 bytes of input into a 81-byte buffer and place a null
    //   terminator at the next slot. This mimics having space for up to 80 chars
    //   plus terminating NUL in the C buffer. Overlong input is thus truncated,
    //   which is a safe approximation of the original (unsafe) behavior.
    // - We operate on raw bytes to match the original C byte-wise operations,
    //   including preservation of NUL bytes if present in the input (they will
    //   terminate processing as in the C code).
    // - If EOF is encountered immediately (no bytes read), we treat it as an
    //   empty line. The original C code would call gets which would return NULL
    //   and leave the buffer indeterminate (UB); safely treating it as empty is
    //   a pragmatic choice.
    //
    // The program preserves output formatting: exactly one newline is written
    // at the end, matching puts() behavior.

    // Read up to and including a newline from stdin into a byte buffer.
    let mut stdin = io::stdin();
    let mut line: Vec<u8> = Vec::new();
    // read_until returns the number of bytes read; we ignore errors and treat them as EOF/premature end.
    let _ = stdin.read_until(b'\n', &mut line);

    // Remove trailing newline if present (gets does not store the newline).
    if line.last() == Some(&b'\n') {
        line.pop();
    }

    // Prepare a fixed-size buffer like `char ch[81]` in C.
    // We'll allow up to 80 bytes of input, then place a NUL byte at the next position.
    let mut ch = [0u8; 81];

    // Copy at most 80 bytes into ch, to leave space for the NUL terminator.
    let copy_len = if line.len() <= 80 { line.len() } else { 80 };
    ch[..copy_len].copy_from_slice(&line[..copy_len]);
    // Explicitly ensure null termination at position copy_len (within bounds 0..=80).
    ch[copy_len] = 0;

    // Process bytes until the first NUL (like the for loop in C: for(i = 0; ch[i] != '\0'; ++i))
    let mut i = 0usize;
    while i < ch.len() {
        let b = ch[i];
        if b == 0 {
            break;
        }
        // If 'a' <= ch[i] <= 'z', subtract 32. Operate on raw bytes to emulate C's char arithmetic.
        if b >= b'a' && b <= b'z' {
            // subtraction with wrapping is not needed here because b >= 'a' ensures result is valid ASCII.
            ch[i] = b - 32;
        }
        i += 1;
    }

    // Write the bytes up to (but not including) the first NUL, then a newline (puts adds a newline).
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    // Write the string bytes (if any).
    if i > 0 {
        let _ = handle.write_all(&ch[..i]);
    }
    // puts always writes a terminating newline.
    let _ = handle.write_all(b"\n");
    // flush to ensure output is emitted promptly.
    let _ = handle.flush();
}
