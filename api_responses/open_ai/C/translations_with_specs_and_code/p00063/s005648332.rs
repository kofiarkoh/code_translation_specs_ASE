// palcount.rs
//
// Reads whitespace-delimited tokens from stdin and counts how many are palindromes
// (byte-wise equality with their reverse). Prints the count as a decimal integer
// followed by a newline.
//
// This implementation follows the specification exactly with respect to:
// - tokenization on ASCII whitespace (space, tab, newline, vertical tab,
//   form-feed, carriage return),
// - byte-wise comparison (so non-UTF-8 bytes are handled as-is),
// - printing a single integer and a newline, and
// - on read errors we treat them as end-of-input and proceed with whatever bytes
//   were read so far.
//
// Note: The original C program used a fixed buffer size (N=100) and declared
// behavior undefined for tokens exceeding that capacity. The specification
// treats that as a precondition rather than a required enforcement. This Rust
// program does not impose a fixed-token-size limit (it will accept longer
// tokens); that relaxes the original C program's undefined behavior, which is
// not a conflict with the specification (the spec allows the program to assume
// tokens fit into its buffers). To preserve exact byte-wise behavior and to
// correctly handle possible non-UTF-8 input, we operate on raw bytes.

use std::io::{self, Read};

fn is_ascii_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b /* VT */ | 0x0c /* FF */)
}

fn is_palindrome_bytes(s: &[u8]) -> bool {
    let mut i = 0usize;
    let mut j = s.len();
    if j == 0 {
        return true; // empty token (shouldn't occur with split rules) is a palindrome
    }
    // j is len; compare s[i] with s[j-1], etc.
    while i < j {
        if s[i] != s[j - 1] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

fn main() {
    // Read all available bytes from stdin. On error, proceed with whatever was read.
    let mut buf = Vec::new();
    let _ = io::stdin().lock().read_to_end(&mut buf);
    let mut count: usize = 0;

    let mut start: usize = 0;
    let len = buf.len();

    // Iterate through bytes and split on ASCII whitespace.
    // We go to <= len so that the final token (if any) gets processed when i == len.
    for i in 0..=len {
        if i == len || is_ascii_whitespace(buf[i]) {
            if start < i {
                let token = &buf[start..i];
                if is_palindrome_bytes(token) {
                    count += 1;
                }
            }
            start = i + 1;
        }
    }

    // Print the count followed by a newline, and exit normally (status 0).
    println!("{}", count);
}
