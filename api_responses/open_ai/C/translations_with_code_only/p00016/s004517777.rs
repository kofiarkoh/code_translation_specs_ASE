// translation.rs
//
// Translated from the provided C program.
// Notes:
// - The original C used uninitialized local variables (x, y, d).
//   That is undefined behavior in C. The likely intended semantics are to start
//   at the origin facing "north" (d == 0). To produce deterministic behavior
//   and match typical intent, x, y, d are initialized to 0.0 here.
// - Input parsing emulates scanf("%lf,%lf", &mov, &dir) semantics:
//   * For the first %lf in the pair, leading whitespace is skipped.
//   * There must be a literal ',' immediately after the first number (no
//     intervening whitespace). After the comma, the second %lf will skip
//     leading whitespace.
//   * The loop stops when both mov and dir are exactly 0.0 (as in C double equality).
// - Output formatting matches the C program: two lines, each an integer obtained
//   by casting the final x and y doubles to int (Rust `as i32` truncates toward zero).

use std::io::{self, Read};
use std::f64::consts::PI;

/// Try to parse a floating point number from `s` starting at `pos`.
/// If `skip_leading_ws` is true, leading ASCII whitespace is skipped first.
/// Returns Some((value, next_pos)) on success, or None on failure.
fn parse_float(s: &str, mut pos: usize, skip_leading_ws: bool) -> Option<(f64, usize)> {
    let bytes = s.as_bytes();
    let len = bytes.len();

    if skip_leading_ws {
        while pos < len && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\n' || bytes[pos] == b'\r' || bytes[pos] == 0x0b || bytes[pos] == 0x0c) {
            pos += 1;
        }
    }

    if pos >= len {
        return None;
    }

    let start = pos;

    // Optional sign
    if bytes[pos] == b'+' || bytes[pos] == b'-' {
        pos += 1;
        if pos >= len {
            // sign only, not a number
            return None;
        }
    }

    // Check for special values "inf", "infinity", "nan" (case-insensitive)
    // We will check them first if present at current position.
    // Build a lowercase view for checking a few characters without allocation.
    let remaining = &s[start..];
    let rem_lc = remaining.to_ascii_lowercase();
    if rem_lc.starts_with("inf") {
        // Accept "inf" or "infinity"
        let take = if rem_lc.starts_with("infinity") { 8 } else { 3 };
        let substr = &s[start..start + take];
        if let Ok(v) = substr.parse::<f64>() {
            return Some((v, start + take));
        } else {
            // Fallthrough to more general scanning below if parse fails.
        }
    } else if rem_lc.starts_with("nan") {
        // Accept "nan" or "nan(...)" - let Rust parse it from the "nan" prefix + optional payload
        // We'll consume "nan" then optionally parentheses until the closing ')'.
        let mut end = start + 3;
        if end < len && bytes[end] == b'(' {
            // find matching ')'
            end += 1;
            while end < len && bytes[end] != b')' {
                end += 1;
            }
            if end < len && bytes[end] == b')' {
                end += 1;
            }
        }
        let substr = &s[start..end];
        if let Ok(v) = substr.parse::<f64>() {
            return Some((v, end));
        }
    }

    // General numeric parsing:
    // integer part digits
    let mut any_digits = false;
    while pos < len && (bytes[pos] as char).is_ascii_digit() {
        any_digits = true;
        pos += 1;
    }

    // fractional part
    if pos < len && bytes[pos] == b'.' {
        pos += 1;
        while pos < len && (bytes[pos] as char).is_ascii_digit() {
            any_digits = true;
            pos += 1;
        }
    }

    // exponent part
    if any_digits && pos < len && (bytes[pos] == b'e' || bytes[pos] == b'E') {
        let exp_pos = pos;
        pos += 1;
        if pos < len && (bytes[pos] == b'+' || bytes[pos] == b'-') {
            pos += 1;
        }
        let exp_digits_start = pos;
        while pos < len && (bytes[pos] as char).is_ascii_digit() {
            pos += 1;
        }
        if pos == exp_digits_start {
            // No digits after exponent marker; rollback exponent
            pos = exp_pos;
        }
    }

    if any_digits {
        let substr = &s[start..pos];
        if let Ok(v) = substr.parse::<f64>() {
            return Some((v, pos));
        } else {
            return None;
        }
    }

    // If we didn't see digits but there was a sign, and maybe special values didn't match,
    // attempt to parse using Rust's parse on a reasonable span (fallback).
    // Attempt to consume characters that are typical in float representations.
    let mut pos2 = start;
    while pos2 < len {
        let c = bytes[pos2] as char;
        if c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.' || c == '(' || c == ')' {
            pos2 += 1;
        } else {
            break;
        }
    }
    if pos2 > start {
        let substr = &s[start..pos2];
        if let Ok(v) = substr.parse::<f64>() {
            return Some((v, pos2));
        }
    }

    None
}

fn main() {
    // Initialize to 0.0 to match the intended (typical) behavior of the original program.
    // The original C code had uninitialized x,y,d which is undefined; initializing here
    // yields deterministic, expected behavior.
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut d: f64 = 0.0;

    // Read entire stdin into a string for parsing
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    let mut pos = 0usize;
    let len = input.len();

    loop {
        // For the first %lf: skip leading whitespace
        let res1 = parse_float(&input, pos, true);
        let (mov, after1) = match res1 {
            Some(v) => v,
            None => break,
        };

        // After parsing first number, the next character MUST be a literal ','
        // (scanf("%lf,%lf") requires that; whitespace between the number and comma would make match fail)
        if after1 >= len || input.as_bytes()[after1] != b',' {
            // No literal comma -> stop parsing (scanf would stop/fail)
            break;
        }
        // advance past comma
        pos = after1 + 1;

        // For the second %lf: scanf would skip leading whitespace
        let res2 = parse_float(&input, pos, true);
        let (dir, after2) = match res2 {
            Some(v) => v,
            None => break,
        };

        pos = after2;

        // emulate the loop condition: continue while mov != 0 || dir != 0
        if mov == 0.0 && dir == 0.0 {
            break;
        }

        // perform the motion update: use radians conversion
        let rad = d * PI / 180.0;
        x += rad.sin() * mov;
        y += rad.cos() * mov;
        d += dir;
    }

    // Print integers by casting as in C ((int)x). Rust `as i32` truncates toward zero.
    let ix = x as i32;
    let iy = y as i32;
    // Preserve exact printf formatting: "%d\n%d\n"
    println!("{}", ix);
    println!("{}", iy);
}
