// caesar_translation.rs
//
// Translated from the provided C program. This Rust program preserves the
// original C semantics as closely as possible:
// - Input is read as bytes and truncated to at most 80 bytes (C used a
//   buffer of size 81 and fgets).
// - Byte-level Caesar shifting matches the C code's numeric checks against 122
//   and subtraction of 26 when exceeding 122.
// - Space ' ' and period '.' are left unchanged.
// - Substring searches ("the", "this", "that") are done on the transformed
//   byte buffer (C used strstr on a C string).
// - Printing emulates printf("%s\n", caesar_str): bytes up to the first NUL
//   (0) are written, then a newline is written.
//
// The implementation avoids unsafe code and works with raw bytes to mimic C's
// char/byte semantics (not UTF-8-aware slicing).

use std::io::{self, Read, Write};

fn caesar(input: &[u8], shift: i32) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    for &b in input {
        if b == b' ' {
            out.push(b' ');
        } else if b == b'.' {
            out.push(b'.');
        } else {
            let val = (b as i32) + shift;
            if val <= 122 {
                out.push(val as u8);
            } else {
                out.push((val - 26) as u8);
            }
        }
    }
    out
}

fn contains_subslice(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.len() == 0 {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }
    haystack.windows(needle.len()).any(|w| w == needle)
}

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut input = Vec::new();

    // Read one line (up to and including newline). This corresponds to fgets behavior.
    stdin.read_until(b'\n', &mut input)?;

    // Truncate to at most 80 bytes to match fgets(buffer, 81, stdin) which reads up to 80 chars.
    if input.len() > 80 {
        input.truncate(80);
    }

    let mut stdout = io::stdout();

    for shift in 0..26 {
        let transformed = caesar(&input, shift);

        // Order matters: check "the" first, then "this", then "that",
        // matching the C code's if / else if chain.
        let mut should_print = false;
        if contains_subslice(&transformed, b"the") {
            should_print = true;
        } else if contains_subslice(&transformed, b"this") {
            should_print = true;
        } else if contains_subslice(&transformed, b"that") {
            should_print = true;
        }

        if should_print {
            // Emulate printf("%s\n", caesar_str): print bytes up to first NUL (0), then newline.
            let upto_nul = match transformed.iter().position(|&x| x == 0) {
                Some(pos) => &transformed[..pos],
                None => &transformed[..],
            };
            stdout.write_all(upto_nul)?;
            stdout.write_all(b"\n")?;
        }
    }

    Ok(())
}
