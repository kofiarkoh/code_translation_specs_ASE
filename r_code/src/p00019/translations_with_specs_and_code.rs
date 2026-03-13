// factorial.rs
//
// Translated from the provided C program according to the SPEC.
// Behavior summary (per SPEC):
// - Read a single signed int from stdin (scans like scanf("%d", &i)):
//   it skips leading whitespace, accepts an optional '+'/'-' sign, then consumes digits.
// - If a valid integer is read and it's > 0, print n! (product n * (n-1) * ... * 1).
// - If the integer is <= 0, print 1.
// - Output is the decimal representation of the resulting int followed by a newline.
// - The SPEC assumes the input is a valid signed int and that factorial fits in the
//   signed int range. Behavior on parse failure or overflow is unspecified by SPEC.
// Implementation notes:
// - We emulate scanf("%d", ...) parsing: skip leading whitespace, optional sign, then digits.
// - To avoid panics in debug builds on overflow (and to remain deterministic),
//   multiplication uses wrapping arithmetic (i32::wrapping_mul). The SPEC states
//   signed overflow is undefined in C; wrapping here is a safe, well-defined choice
//   that avoids runtime panics in Rust while preserving the loop's intent.
// - If parsing fails (no integer token present), we simply exit without printing,
//   matching the SPEC's precondition that a valid integer is provided (the original C
//   program's behavior is undefined in that case).
use std::io::{self, Read};

fn main() {
    // Read all stdin into a string (we only need to scan once).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we cannot read stdin for some reason, follow SPEC assumptions:
        // behavior is unspecified; simply exit.
        return;
    }

    // Emulate scanf("%d", &i):
    // - skip leading whitespace
    // - optional '+' or '-' sign
    // - then a sequence of digits (at least one)
    let s = input.as_str();
    let mut chars = s.char_indices();

    // Skip leading whitespace
    let mut start = None;
    while let Some((idx, ch)) = chars.next() {
        if !ch.is_whitespace() {
            start = Some(idx);
            break;
        }
    }
    let start = match start {
        Some(pos) => pos,
        None => {
            // No non-whitespace characters => scanf would fail; SPEC assumes valid input.
            return;
        }
    };

    // Examine optional sign
    let mut end = start;
    let bytes = s.as_bytes();
    if let Some(&b) = bytes.get(end) {
        if b == b'+' || b == b'-' {
            end += 1;
        }
    }

    // Consume digits
    let mut digit_count = 0usize;
    while let Some(&b) = bytes.get(end) {
        if (b'0'..=b'9').contains(&b) {
            digit_count += 1;
            end += 1;
        } else {
            break;
        }
    }

    if digit_count == 0 {
        // No digits after optional sign => scanf would fail; SPEC assumes valid input.
        return;
    }

    // Extract the integer token substring
    let token = &s[start..end];

    // Parse as i32 (C signed int is the SPEC reference; using i32 here).
    let i_val: i32 = match token.parse() {
        Ok(v) => v,
        Err(_) => {
            // Parsing failed (e.g., out of range). SPEC assumes input fits in signed int;
            // for invalid parse, behavior is unspecified. We choose to exit.
            return;
        }
    };

    // Compute factorial as in the C code:
    // j = 1; for (; i > 0; i--) j *= i;
    // Use wrapping_mul to avoid panic on overflow (deterministic behavior).
    let mut i = i_val;
    let mut j: i32 = 1;
    while i > 0 {
        j = j.wrapping_mul(i);
        i -= 1;
    }

    // Print result followed by a newline, exactly as in the C program.
    // Using println! ensures the trailing newline.
    println!("{}", j);
}
