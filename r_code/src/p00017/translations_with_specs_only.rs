// caesar_bruteforce.rs
//
// Translation of the provided Specification into Rust.
// Notes on faithful translations and corner cases:
// - Input is read as raw bytes up to and including the first newline (like fgets).
// - At most 80 bytes are kept (fgets with buffer length 81 behavior).
// - Transformation operates on bytes. Lowercase ASCII 'a'..'z' are rotated modulo 26.
//   Space (0x20) and period (0x2E) are preserved.
//   All other bytes are transformed by addition of the shift modulo 256 (wrapping add).
// - For each shift 0..25 we produce a NUL-terminated transformed buffer (kept internally).
// - Substring search is performed on the transformed bytes before the NUL terminator.
//   The search checks "the", then "this", then "that" (in that order), and prints
//   the transformed string once for that shift if any check succeeds.
// - Printing emulates printf("%s\n", ...): the transformed bytes (up to NUL) are written
//   raw to stdout, then a newline byte is written unconditionally.
//
// The program intentionally treats I/O errors similarly to the C specification which
// "does not check" fgets return: the initial read result is not explicitly handled.
// All operations are kept in safe Rust.

use std::io::{self, BufRead, Write};

fn main() {
    // Read up to and including a newline from stdin (like fgets does).
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input: Vec<u8> = Vec::new();

    // read_until returns io::Result<usize>. The original C spec did not check fgets'
    // return value; to mirror that, we do not branch on the Result here (we ignore errors).
    // This keeps behavior close to "undefined" on read failure while remaining compilable.
    let _ = handle.read_until(b'\n', &mut input);

    // Truncate to at most 80 bytes (fgets with buffer length 81 would keep up to 80 chars).
    if input.len() > 80 {
        input.truncate(80);
    }

    // Predefined target substrings in order.
    const TARGET1: &[u8] = b"the";
    const TARGET2: &[u8] = b"this";
    const TARGET3: &[u8] = b"that";

    let mut stdout = io::stdout();

    // For each shift s in 0..25
    for s in 0u8..=25u8 {
        // Build transformed buffer of same length as input, then append a NUL byte.
        let n = input.len();
        let mut transformed: Vec<u8> = Vec::with_capacity(n + 1);

        for &b in &input {
            let tb: u8 = if b == b' ' || b == b'.' {
                // Preserve space and period.
                b
            } else if (b'a'..=b'z').contains(&b) {
                // Lowercase letters rotate modulo 26.
                let offset = b - b'a';
                let shifted = ((offset as u8).wrapping_add(s)) % 26;
                b'a' + shifted
            } else {
                // Other bytes: add shift modulo 256 (wrapping add).
                b.wrapping_add(s)
            };
            transformed.push(tb);
        }

        // Append explicit NUL terminator (as C would).
        transformed.push(0u8);

        // For substring checks, examine the bytes before the terminating NUL.
        let content = &transformed[..n]; // excludes the trailing NUL

        // Helper closure to check for a byte-slice substring.
        let contains_sub = |hay: &[u8], needle: &[u8]| -> bool {
            if needle.len() == 0 || hay.len() < needle.len() {
                return false;
            }
            hay.windows(needle.len()).any(|w| w == needle)
        };

        // Check in order: "the", then "this", then "that".
        let matched = if contains_sub(content, TARGET1) {
            true
        } else if contains_sub(content, TARGET2) {
            true
        } else if contains_sub(content, TARGET3) {
            true
        } else {
            false
        };

        if matched {
            // Emulate printf("%s\n", transformed);
            // Write the bytes up to NUL, then write a newline byte.
            // Ignore write errors to remain close to the original unchecked behavior.
            let _ = stdout.write_all(content);
            let _ = stdout.write_all(b"\n");
        }
    }

    // Exit with status 0 on normal completion.
    // Let main return; process will exit with 0.
}
