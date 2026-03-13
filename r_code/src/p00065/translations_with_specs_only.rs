// duplicates.rs
//
// Translated from the given Specification:
// - Reads zero or more pairs "<number>,<ac>" from stdin.
// - Collects the first integers (number) from each pair (ignores ac).
// - Up to 10000 pairs are stored (consistent with the Specification).
// - Sorts the collected first integers in non-decreasing order.
// - Prints each integer that appears at least twice with its total count as "v count\n".
// Notes about parsing:
// - Parsing emulates scanf("%d,%d") semantics with respect to whitespace and the literal comma:
//   * Leading whitespace before the first integer is allowed (it is skipped).
//   * The comma must immediately follow the digits of the first integer (no intervening non-digit characters).
//   * Whitespace after the comma and before the second integer is allowed (it is skipped).
// - If input deviates from the required pattern, parsing stops (per the Specification input is assumed correct).
//
// The code prefers safe, idiomatic Rust and follows the Specification behavior as closely as reasonable.

use std::io::{self, Read};

fn is_ascii_digit(b: u8) -> bool {
    b'0' <= b && b <= b'9'
}

fn is_ascii_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\n' | b'\t' | b'\r' | 0x0b | 0x0c)
}

fn main() {
    // Read all stdin into a buffer.
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        // On I/O error, per Specification there are no explicit error messages;
        // simply exit (main returns non-zero implicitly only on panic; we avoid panicking).
        return;
    }

    const MAX_PAIRS: usize = 10000;
    let mut first_vals: Vec<i32> = Vec::with_capacity(MAX_PAIRS);

    let mut pos: usize = 0;
    let len = input.len();

    // Parsing loop: emulate repeated scanf("%d,%d")
    'outer: loop {
        // Skip leading whitespace before first integer (scanf's %d skips whitespace)
        while pos < len && is_ascii_whitespace(input[pos]) {
            pos += 1;
        }
        if pos >= len {
            break; // EOF
        }

        // Parse first integer: optional sign, then at least one digit.
        let start = pos;
        // Optional sign
        if pos < len && (input[pos] == b'+' || input[pos] == b'-') {
            pos += 1;
        }
        let digits_start = pos;
        while pos < len && is_ascii_digit(input[pos]) {
            pos += 1;
        }
        if digits_start == pos {
            // No digits for first integer -> malformed input per Specification.
            // We stop parsing further (Specification assumes well-formed input).
            break;
        }
        let first_slice = &input[start..pos];
        let first_str = match std::str::from_utf8(first_slice) {
            Ok(s) => s,
            Err(_) => break, // malformed UTF-8 (shouldn't happen for ASCII digits/sign)
        };
        // The literal comma must immediately follow the digits (no whitespace).
        if pos >= len || input[pos] != b',' {
            // Malformed: comma not found in the correct position.
            break;
        }
        // Consume comma
        pos += 1;

        // After comma, whitespace before second integer is allowed (scanf's %d skips whitespace)
        while pos < len && is_ascii_whitespace(input[pos]) {
            pos += 1;
        }
        if pos >= len {
            // EOF before second integer -> malformed
            break;
        }

        // Parse second integer (we will ignore its value, but must consume it).
        let second_start = pos;
        if pos < len && (input[pos] == b'+' || input[pos] == b'-') {
            pos += 1;
        }
        let second_digits_start = pos;
        while pos < len && is_ascii_digit(input[pos]) {
            pos += 1;
        }
        if second_digits_start == pos {
            // No digits for second integer -> malformed
            break;
        }
        // We have consumed a pair successfully; now parse first integer text into i32.
        let first_value: i32 = match first_str.parse() {
            Ok(v) => v,
            Err(_) => {
                // If it doesn't fit in i32 or otherwise fails, stop parsing (Specification requires C int fits).
                break;
            }
        };

        // Store first integer if capacity allows.
        if first_vals.len() >= MAX_PAIRS {
            // The original C program allocated fixed arrays of size 10000 and would exhibit undefined behavior
            // if more pairs are supplied. Here we choose the narrowest reasonable behavior: stop reading further
            // pairs once MAX_PAIRS is reached and proceed to processing.
            break;
        }
        first_vals.push(first_value);

        // Continue to next pair. Pairs may be separated by whitespace/newlines; loop handles that.
    }

    // Sort the collected first-integers in non-decreasing order.
    // The Specification indicates an O(n^2) sort in the original; we implement a simple selection sort
    // which is O(n^2) to be faithful to the described complexity.
    let n = first_vals.len();
    for i in 0..n {
        let mut min_idx = i;
        let mut j = i + 1;
        while j < n {
            if first_vals[j] < first_vals[min_idx] {
                min_idx = j;
            }
            j += 1;
        }
        if min_idx != i {
            first_vals.swap(i, min_idx);
        }
    }

    // Scan sorted list and print values that appear at least twice.
    let mut i = 0;
    while i < n {
        let v = first_vals[i];
        let mut j = i + 1;
        while j < n && first_vals[j] == v {
            j += 1;
        }
        let count = (j - i) as i32;
        if count >= 2 {
            // Print "v count\n"
            println!("{} {}", v, count);
        }
        i = j;
    }

    // Program exits with code 0 on normal termination (implicit on successful return from main).
}
