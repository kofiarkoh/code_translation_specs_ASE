// translation of the provided C program to Rust following the SPEC.
// Notes:
// - The SPEC allows empty lines; the original C used scanf("%[^\n]\n", ...) which
//   would fail on empty lines. This Rust implementation supports empty lines
//   (matches the SPEC) — this is an explicit, documented divergence from the C code.
//
// Behavior: Read an integer n (first token), then read n newline-terminated lines.
// For each line and for a,b in 0..=99, search for a 4-char window that maps to
// "this" or "that" under the affine mapping. If found, print the whole line with
// affine mapping applied to every non-space character, then exit (return 0).
use std::io::{self, Read};

fn affine_byte(c: u8, a: i32, b: i32) -> u8 {
    // Compute (a * (c - 'a') + b) % 26 + 'a', with a positive modulo result.
    // Operate in i32 to avoid overflow for the given ranges.
    let val = a * ((c as i32) - ('a' as i32)) + b;
    let m = ((val % 26) + 26) % 26;
    (m + ('a' as i32)) as u8
}

fn this_check(bytes: &[u8], i: usize, a: i32, b: i32) -> bool {
    // Precondition: i >= 3 and i < bytes.len()
    affine_byte(bytes[i], a, b) == b's'
        && affine_byte(bytes[i - 1], a, b) == b'i'
        && affine_byte(bytes[i - 2], a, b) == b'h'
        && affine_byte(bytes[i - 3], a, b) == b't'
}

fn that_check(bytes: &[u8], i: usize, a: i32, b: i32) -> bool {
    // Precondition: i >= 3 and i < bytes.len()
    affine_byte(bytes[i], a, b) == b't'
        && affine_byte(bytes[i - 1], a, b) == b'a'
        && affine_byte(bytes[i - 2], a, b) == b'h'
        && affine_byte(bytes[i - 3], a, b) == b't'
}

fn search(line: &str, a: i32, b: i32) -> bool {
    let bytes = line.as_bytes();
    let len = bytes.len();
    if len < 4 {
        return false;
    }
    for i in 3..len {
        if bytes[i] == b' ' {
            continue;
        }
        let y = affine_byte(bytes[i], a, b);
        if (y == b't' && that_check(bytes, i, a, b))
            || (y == b's' && this_check(bytes, i, a, b))
        {
            return true;
        }
    }
    false
}

fn print_affine_line(s: &str, a: i32, b: i32) {
    let mut out = String::with_capacity(s.len() + 1);
    for &byte in s.as_bytes() {
        if byte == b' ' {
            out.push(' ');
        } else {
            out.push(affine_byte(byte, a, b) as char);
        }
    }
    // Print the transformed line followed by a newline (exactly one newline).
    println!("{}", out);
}

fn main() {
    // Read entire stdin to a string to reliably handle empty lines.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // Per SPEC, behavior on malformed input is unspecified; exit silently.
        return;
    }

    let bytes = input.as_bytes();
    let mut idx = 0;
    let nbytes = bytes.len();

    // Skip leading whitespace
    while idx < nbytes && bytes[idx].is_ascii_whitespace() {
        idx += 1;
    }

    // Parse integer token (sequence of ASCII digits)
    let start = idx;
    while idx < nbytes && (bytes[idx] as char).is_ascii_digit() {
        idx += 1;
    }
    if start == idx {
        // No integer found; unspecified behaviour per SPEC. Exit silently.
        return;
    }
    let n_token = &input[start..idx];
    let n: usize = match n_token.parse() {
        Ok(v) => v,
        Err(_) => return, // malformed integer, exit silently per SPEC
    };

    // After the integer, consume a single newline if present (support CRLF too).
    if idx < nbytes {
        if bytes[idx] == b'\r' {
            idx += 1;
            if idx < nbytes && bytes[idx] == b'\n' {
                idx += 1;
            }
        } else if bytes[idx] == b'\n' {
            idx += 1;
        } else if bytes[idx].is_ascii_whitespace() {
            // If there's some other whitespace, consume exactly one (best-effort;
            // original C used "%d\n" which can be problematic; SPEC is authoritative).
            idx += 1;
        }
    }

    // The remainder, split on '\n', preserving empty lines as empty strings.
    let remainder = &input[idx..];
    let mut lines: Vec<&str> = remainder.split('\n').collect();

    // If input ends with a trailing newline, split produces a trailing empty element.
    // That matches the idea of newline-terminated lines. We will take the first n elements,
    // padding with empty strings if there are fewer than n (best-effort; SPEC assumes n lines exist).
    if lines.len() < n {
        // Pad with empty strings to allow processing; original C would block/undefined,
        // but we choose to stop early per available input.
        lines.resize(n, "");
    }

    for line_idx in 0..n {
        let line = lines[line_idx];
        // For each a,b in 0..99 inclusive
        let mut found = false;
        for a in 0..100 {
            for b in 0..100 {
                if search(line, a as i32, b as i32) {
                    print_affine_line(line, a as i32, b as i32);
                    // Per SPEC, terminate immediately after printing the transformed line.
                    return;
                }
            }
            if found {
                break;
            }
        }
    }

    // If no match found after all lines and parameter pairs, exit normally (no output).
}
