// apple_peach_swap.rs
//
// Reads a single line from stdin (up to 1001 bytes of content, matching the original C buffer capacity),
// scans left-to-right and swaps every exact 5-byte occurrence of:
//   "apple" <-> "peach"
//   "Apple" <-> "Peach"
// Replacements are applied in-place and immediately affect subsequent matches.
//
// Notes on translation decisions:
// - The SPEC requires scanning up to the last position where a 5-char substring can begin.
//   The original C code used `for(i=0;i<n-5;i++)` (with `char` indices), which would skip the final
//   possible start index. This Rust translation follows the SPEC (i.e., includes the final start index).
// - We limit the input to 1001 bytes (truncate if needed) to mirror the original C buffer capacity (fgets with size 1002).
// - Implemented in safe, idiomatic Rust. No `unsafe` used.
// - If no input is available, the program exits gracefully (prints nothing) rather than exhibiting undefined behavior.
// - Output formatting: the modified buffer is printed exactly as-is (no extra newline added).

use std::io;

fn main() {
    // Read one line from stdin into a String.
    let mut input = String::new();
    // If reading fails or EOF is reached and nothing is read, behave gracefully (print nothing and exit).
    // This is slightly more defined than the original C program's undefined behavior on fgets failure,
    // but it preserves the observable effect for normal inputs.
    if io::stdin().read_line(&mut input).is_err() || input.is_empty() {
        // Print nothing and exit with status 0.
        return;
    }

    // Work with bytes to perform byte-wise (ASCII-compatible) comparisons and in-place mutations.
    let mut bytes = input.into_bytes();

    // Truncate to 1001 bytes of content to match the original C program's buffer limit (fgets with size 1002).
    if bytes.len() > 1001 {
        bytes.truncate(1001);
    }

    let n = bytes.len();

    // Only attempt replacements when there is at least room for a 5-byte substring.
    if n >= 5 {
        // Iterate from 0 up to and including the last index where a 5-byte substring can begin.
        // This follows the SPEC. (The C code's loop condition omitted the final valid start.)
        for i in 0..=n - 5 {
            // Check the current 5-byte window and replace immediately if it matches one of the target words.
            // The order of checks is not significant for correctness because we match against the current content
            // at position i on each iteration and apply at most one replacement at that position.
            let window = &bytes[i..i + 5];
            if window == b"apple" {
                bytes[i..i + 5].copy_from_slice(b"peach");
            } else if window == b"peach" {
                bytes[i..i + 5].copy_from_slice(b"apple");
            } else if window == b"Apple" {
                bytes[i..i + 5].copy_from_slice(b"Peach");
            } else if window == b"Peach" {
                bytes[i..i + 5].copy_from_slice(b"Apple");
            }
        }
    }

    // Convert bytes back to a String for printing. Since we only substitute ASCII bytes, this should be valid UTF-8.
    // Use a fallback to lossy conversion just in case the input contained invalid UTF-8.
    let out = String::from_utf8(bytes).unwrap_or_else(|e| String::from_utf8_lossy(&e.into_bytes()).into_owned());

    // Print the buffer as-is (do not add an extra newline). If the input contained a newline, it is preserved.
    print!("{}", out);
}
