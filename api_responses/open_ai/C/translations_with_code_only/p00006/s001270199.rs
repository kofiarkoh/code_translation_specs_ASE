// Translated from the provided C program.
// Notes on translation choices (to preserve behavior safely):
// - The original C code used fixed-size char arrays of length 21 (presumably 20 chars + '\0').
//   It used scanf("%s", ...) which can overflow the buffer in C if the input is too long.
//   To avoid undefined behavior in Rust while staying faithful to the intended buffer size,
//   we copy at most 20 bytes and ensure a terminating NUL byte.
// - The C code reversed the string bytes (not characters); we operate on bytes as well,
//   matching C's byte-wise semantics for ASCII or arbitrary byte input.
// - The C program printed the reversed string with no trailing newline; we preserve that.
//
// This Rust program reads the first whitespace-delimited token from stdin and prints its
// reversal (up to 20 bytes), with no newline.

use std::io::{self, Read};

fn main() {
    const CAP: usize = 21; // like char str[21] in C: room for 20 chars + NUL

    // Read entire stdin and take the first whitespace-delimited token (equivalent to scanf("%s", ...))
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let token = match input.split_whitespace().next() {
        Some(t) => t,
        None => return, // no token read, behave like C which would do nothing / exit
    };

    let bytes = token.as_bytes();

    // Emulate C's fixed buffer: copy at most CAP-1 bytes and write a terminating NUL.
    let mut str_buf = [0u8; CAP];
    let max_copy = CAP - 1;
    let n = bytes.len().min(max_copy);
    str_buf[..n].copy_from_slice(&bytes[..n]);
    str_buf[n] = 0;

    // Compute length using C-like strlen (position of first NUL)
    let x = str_buf.iter().position(|&c| c == 0).unwrap_or(CAP);

    // Prepare output buffer and fill by reversing the first x bytes
    let mut strsub = [0u8; CAP];
    for i in 0..x {
        strsub[i] = str_buf[x - 1 - i];
    }
    // Ensure NUL termination (the original C code omitted this write explicitly, but
    // printing a C string requires it; setting it here matches the intended behavior).
    if x < CAP {
        strsub[x] = 0;
    }

    // Print bytes up to the first NUL, with no trailing newline.
    let out_end = strsub.iter().position(|&c| c == 0).unwrap_or(CAP);
    let out_bytes = &strsub[..out_end];
    print!("{}", String::from_utf8_lossy(out_bytes));
}
