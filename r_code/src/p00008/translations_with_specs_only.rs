// count_4digit_sums.rs
//
// Translates the provided specification into a Rust program.
//
// Behavior notes (faithfully following the specification):
// - The program reads stdin as a sequence of bytes and emulates the C scanf("%d", &n)
//   semantics as described in the specification:
//     * Leading ASCII whitespace is skipped.
//     * If the next non-whitespace sequence is an optional sign followed by digits,
//       those characters are consumed and parsed as an integer.
//     * If the next non-whitespace character is not the start of an integer
//       (e.g. a letter), then parsing fails with "no match" and the token is NOT
//       consumed. According to the specification, the original C program would
//       then repeatedly attempt to read the same invalid token, resulting in an
//       effectively infinite loop. This program reproduces that behavior: on a
//       parse mismatch the program does not consume input and loops forever
//       (no output is produced).
// - For each successfully parsed integer n, the program prints one line with the
//   number of ordered 4-tuples (a,b,c,d) in {0..9}^4 with sum == n.
// - If EOF is reached before a token is found, the program exits normally with code 0.
//
// Implementation choices:
// - For simplicity and to allow not-consuming-on-mismatch behavior, the entire
//   stdin is read into memory up front and parsed using an index into that buffer.
//   This preserves the observable behavior required by the specification.
// - Counts for sums 0..36 are precomputed by enumerating all 10^4 tuples once,
//   as the specification notes that enumerating up to 10^4 combinations is
//   acceptable and intended.

use std::io::{self, Read};

fn main() {
    // Read all stdin into a buffer so we can emulate scanf's "do not consume on mismatch".
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        // On read error, per spec: if a read error causes scanf to return EOF, the program terminates.
        // In this Rust program, we choose to exit silently (no output), as EOF/read error handling.
        return;
    }

    // Precompute counts for sums 0..36 by enumerating all 10^4 combinations.
    let mut counts = [0u16; 37]; // max count is 10000 < 2^16
    for a in 0u8..=9 {
        for b in 0u8..=9 {
            for c in 0u8..=9 {
                for d in 0u8..=9 {
                    let s = (a as usize) + (b as usize) + (c as usize) + (d as usize);
                    counts[s] = counts[s].saturating_add(1);
                }
            }
        }
    }

    // Cursor into the input buffer.
    let mut pos: usize = 0;
    let len = input.len();

    // Helper closures for byte tests.
    let is_ascii_whitespace = |b: u8| -> bool { b.is_ascii_whitespace() };
    let is_ascii_digit = |b: u8| -> bool { b'0' <= b && b <= b'9' };

    // Emulate loop: repeatedly attempt scanf("%d") semantics until EOF.
    loop {
        // Skip leading ASCII whitespace.
        while pos < len && is_ascii_whitespace(input[pos]) {
            pos += 1;
        }

        // If we've reached EOF after skipping whitespace, terminate normally.
        if pos >= len {
            break;
        }

        // At this point, pos points to the first non-whitespace byte.
        // Check if it starts a valid integer: optional sign followed by at least one digit.
        let start = pos;
        let b = input[pos];

        let mut idx = pos;
        let mut sign = 1i64;

        if b == b'+' || b == b'-' {
            if b == b'-' {
                sign = -1;
            }
            // Need to ensure there's at least one digit after the sign.
            if idx + 1 >= len || !is_ascii_digit(input[idx + 1]) {
                // Mismatch per scanf semantics: do not consume the non-matching token.
                // According to the specification, scanf("%d") would return 0 and the token remains;
                // the original loop condition compares return != EOF and thus the program
                // would continue attempting to read the same invalid token repeatedly.
                // To emulate that behavior, we do not advance pos and loop again (infinite loop).
                // We implement it as a tight loop (same observable behavior as described).
                continue;
            }
            idx += 1; // move past sign to start parsing digits
        } else if !is_ascii_digit(b) {
            // First non-whitespace character is not a digit nor sign: mismatch.
            // Do not consume; loop again (infinite loop behavior).
            continue;
        }

        // Parse digits starting at idx.
        let mut value: i64 = 0;
        let mut any_digit = false;
        while idx < len && is_ascii_digit(input[idx]) {
            any_digit = true;
            // accumulate as i64 to avoid intermediate overflow; spec assumes representable ints
            value = value * 10 + (input[idx] - b'0') as i64;
            idx += 1;
        }

        if !any_digit {
            // No digits found after optional sign -> mismatch, do not consume.
            continue;
        }

        // Successful parse: consume the characters we used (from start up to idx).
        pos = idx;

        // Convert to i32 (C 'int' semantics: spec assumes valid tokens fit). We'll use i64->i32 cast.
        let n_i32 = (sign * value) as i32;

        // According to spec: for any n outside 0..36, print 0.
        // For 0..36, print the precomputed count.
        let out_count: u16 = if (0..=36).contains(&(n_i32 as i32)) {
            counts[n_i32 as usize]
        } else {
            0
        };

        // Print the count followed by newline.
        // Use standard output; per spec one line per successfully read integer.
        println!("{}", out_count);
    }

    // Normal termination with exit code 0 (implicit).
}
