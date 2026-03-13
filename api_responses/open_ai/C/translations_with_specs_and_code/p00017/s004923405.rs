// caesar_search.rs
// Translated from C to Rust according to the provided specification.
// - Reads a single input line (up to 80 bytes, newline included if encountered before limit).
// - For each shift 0..25, produces a Caesar-shifted transform (preserving ' ' and '.').
// - If the transformed bytes contain "the", or if not, "this", or if not, "that" (checked in that order),
//   prints the transformed bytes followed by a newline (an extra newline is appended even if the transformed
//   bytes already contain a newline — matching the C program's printf("%s\n", ...)).
//
// Implementation notes:
// - We operate on raw bytes to exactly preserve the byte-wise shifting semantics from the C code.
// - Input is read byte-by-byte up to 80 bytes or until a newline is read, mimicking fgets(buf, 81, stdin).
// - No `unsafe` is used.
// - For substring checks we search for ASCII byte sequences.
// - Writes to stdout are done using write_all to preserve byte-level output behavior.

use std::io::{self, Read, Write};

fn caesar_transform(input: &[u8], shift: u8) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    for &b in input {
        if b == b' ' {
            out.push(b' ');
        } else if b == b'.' {
            out.push(b'.');
        } else {
            // Perform the same numeric rule as the C code:
            // if (b + shift <= 122) c = b + shift; else c = b + shift - 26;
            // Use signed arithmetic in a wider integer to avoid overflow on non-ASCII bytes.
            let val = (b as i32) + (shift as i32);
            let c = if val <= 122 { val } else { val - 26 };
            out.push(c as u8);
        }
    }
    out
}

fn contains_subslice(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }
    haystack.windows(needle.len()).any(|w| w == needle)
}

fn main() {
    // Read up to 80 bytes (matching fgets with buffer size 81: up to 80 chars + NUL).
    // Stop early if a newline byte is read (include that newline in the buffer).
    let stdin = io::stdin();
    let mut input = Vec::with_capacity(80);
    for byte_result in stdin.lock().bytes() {
        match byte_result {
            Ok(b) => {
                if input.len() >= 80 {
                    break; // mimic truncation by fgets
                }
                input.push(b);
                if b == b'\n' {
                    break; // fgets stops after and includes the newline if it fits
                }
            }
            Err(_) => {
                // The original C program did not check fgets failure; behavior was undefined.
                // Here we stop reading on error and continue with whatever we have (possibly empty).
                break;
            }
        }
    }

    // Prepare patterns as byte slices
    let pat_the = b"the";
    let pat_this = b"this";
    let pat_that = b"that";

    let mut stdout = io::stdout();
    let mut handle = stdout.lock();

    // For each shift 0..25
    for s in 0u8..26u8 {
        let transformed = caesar_transform(&input, s);

        // Check substrings in the specified order: "the", then "this", then "that".
        if contains_subslice(&transformed, pat_the)
            || (!contains_subslice(&transformed, pat_the) && contains_subslice(&transformed, pat_this))
            || (!contains_subslice(&transformed, pat_the) && !contains_subslice(&transformed, pat_this) && contains_subslice(&transformed, pat_that))
        {
            // Write transformed bytes exactly, then an extra newline (matching printf("%s\n", ...)).
            let _ = handle.write_all(&transformed);
            let _ = handle.write_all(b"\n");
            // Ignore write errors as the original C program did not handle stdout errors.
        }
    }
    // Exit implicitly with status 0.
}
