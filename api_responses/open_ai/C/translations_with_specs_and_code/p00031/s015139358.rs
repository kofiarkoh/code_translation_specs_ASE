// weight.rs
//
// Translated from the provided C program, following the SPEC as source-of-truth.
//
// Behavior:
// - Read lines from stdin until EOF.
// - Parse each line with C-like atoi semantics (leading whitespace allowed, optional sign,
//   sequence of digits; if no digits -> 0).
// - For positive integers, print the weights corresponding to set bits 0..9 (1,2,4,...,512),
//   in increasing order, separated by single spaces, terminated by a newline.
// - For zero, negative, or non-numeric lines, print a blank line.
// Note: Per SPEC, inputs with bits >= 10 (i.e., >= 1024) are outside the defined domain.
// This Rust implementation safely considers only bits 0..9 and will ignore higher bits
// (avoiding out-of-bounds access that the original C code could produce).

use std::io::{self, BufRead, Write};

const WEIGHTS: [i32; 10] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512];

/// A minimal implementation of C's atoi semantics:
/// - Skip leading ASCII whitespace
/// - Optional '+' or '-' sign
/// - Parse consecutive ASCII digits; stop at first non-digit
/// - If no digits are found, return 0
/// - Uses saturating arithmetic to avoid overflow
fn atoi_like(s: &str) -> i64 {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    let n = bytes.len();

    // skip leading whitespace
    while i < n && bytes[i].is_ascii_whitespace() {
        i += 1;
    }

    // optional sign
    let mut negative = false;
    if i < n {
        if bytes[i] == b'+' {
            i += 1;
        } else if bytes[i] == b'-' {
            negative = true;
            i += 1;
        }
    }

    // parse digits
    let mut any = false;
    let mut val: i64 = 0;
    while i < n {
        let c = bytes[i];
        if c.is_ascii_digit() {
            any = true;
            // saturating to avoid overflow; exact overflow semantics of C atoi are
            // implementation-defined and not required by SPEC.
            val = val.saturating_mul(10).saturating_add((c - b'0') as i64);
            i += 1;
        } else {
            break;
        }
    }

    if !any {
        0
    } else if negative {
        -val
    } else {
        val
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line_res in stdin.lock().lines() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => break, // on read error, stop (matches EOF behavior)
        };

        let x = atoi_like(&line);

        if x <= 0 {
            // For zero, negative, or non-numeric lines (atoi -> 0), print a blank line.
            // Ensure exactly one newline and no trailing spaces.
            writeln!(stdout).expect("write failed");
            continue;
        }

        // Collect weights for set bits 0..9 (per SPEC). We purposely limit to 10 weights
        // to avoid out-of-bounds indexing; the SPEC marks behavior for bits >= 10 as undefined.
        let mut parts: Vec<String> = Vec::new();
        for i in 0..WEIGHTS.len() {
            if ((x >> i) & 1) != 0 {
                parts.push(WEIGHTS[i].to_string());
            }
        }

        if parts.is_empty() {
            writeln!(stdout).expect("write failed");
        } else {
            writeln!(stdout, "{}", parts.join(" ")).expect("write failed");
        }
    }
}
