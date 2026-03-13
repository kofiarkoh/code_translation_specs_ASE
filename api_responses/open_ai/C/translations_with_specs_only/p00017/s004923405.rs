use std::io::{self, Write};

/// Apply a Caesar-like shift to `src` bytes, writing result into `dest`.
///
/// Semantics follow the provided specification:
/// - dest must have capacity >= src.len() + 1. If not, this function will panic
///   (the original C specification treats insufficient capacity as undefined).
/// - Space (0x20) and period (0x2E) are preserved.
/// - For all other bytes, compute res = (byte as i32) + shift; if res > ASCII 'z' (122),
///   subtract 26 from res. The resulting byte (truncated to u8) is written to dest.
/// - After writing the transformed bytes, a null terminator (0) is written at dest[src.len()].
/// - Returns a mutable slice to `dest`.
fn caesar(src: &[u8], dest: &mut [u8], shift: i32) -> &mut [u8] {
    if dest.len() < src.len() + 1 {
        panic!("dest buffer too small for caesar output");
    }

    for (i, &b) in src.iter().enumerate() {
        if b == b' ' || b == b'.' {
            dest[i] = b;
        } else {
            let mut res = (b as i32) + shift;
            if res > 122 {
                res -= 26;
            }
            dest[i] = res as u8;
        }
    }

    // Null terminator to emulate C-string behavior per specification.
    dest[src.len()] = 0;

    dest
}

fn contains_any(target: &[u8], haystack: &[u8]) -> bool {
    // Simple byte-wise substring search using windows.
    if target.len() == 0 {
        return true;
    }
    if haystack.len() < target.len() {
        return false;
    }
    haystack.windows(target.len()).any(|w| w == target)
}

fn main() {
    // Read one line from stdin. Emulate fgets with max 80 characters.
    let mut input = String::new();
    // If reading fails, propagate the error by panicking (stdin is assumed readable per spec).
    let _ = io::stdin().read_line(&mut input).unwrap();

    // Work with raw bytes to precisely mirror ASCII-based C semantics.
    let mut src_bytes = input.into_bytes();

    // Truncate to at most 80 bytes (emulating fgets truncation behavior).
    if src_bytes.len() > 80 {
        src_bytes.truncate(80);
    }

    let n = src_bytes.len();
    // Allocate destination buffer with room for a null terminator as specified.
    let mut dest = vec![0u8; n + 1];

    // Targets to search for (case-sensitive).
    let targets: [&[u8]; 3] = [b"the", b"this", b"that"];

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Iterate shifts 0 through 25 inclusive.
    for shift in 0..=25 {
        // Apply caesar transform. We pass shift as i32 to match spec semantics.
        let _ = caesar(&src_bytes, &mut dest, shift);

        // We search only within the transformed bytes (excluding the null terminator).
        let haystack = &dest[..n];

        // Check if any target substring is present.
        let mut found = false;
        for &t in &targets {
            if contains_any(t, haystack) {
                found = true;
                break;
            }
        }

        if found {
            // Write the transformed bytes exactly, then a newline (printf("%s\n", ...)).
            // Even if the transformed bytes contain newlines or non-UTF8, we write raw bytes.
            handle.write_all(haystack).unwrap();
            handle.write_all(b"\n").unwrap();
        }
    }

    // Program exits normally with code 0.
}
