// translated from the provided C program
// Notes on translation choices:
// - The original C code used uninitialized local variables x, y, d.
//   That is undefined behavior in C; the intended semantics (and common
//   problem intent) is to start at the origin facing 0 degrees. We therefore
//   initialize x, y, d to 0.0 to emulate the intended behavior.
// - The C code used scanf("%lf,%lf", &mov, &dir). To emulate this reasonably
//   closely we parse the input stream manually, requiring a literal ','
//   between the two floating-point values (no whitespace skipped before the comma,
//   which matches scanf behaviour for a literal in the format string).
//   We do allow the usual components of floating-point literals and use
//   Rust's f64 parser to validate tokens.
// - Casting from double to int in C truncates toward zero; we use trunc()
//   then cast to i32 so the truncation semantics are preserved.
// - This implementation uses only safe Rust.

use std::f64::consts::PI;
use std::io::{self, Read};

fn parse_double(s: &str, idx: &mut usize) -> Option<f64> {
    let bytes = s.as_bytes();
    let n = bytes.len();

    // Skip leading whitespace (scanf's %lf skips leading whitespace).
    while *idx < n && bytes[*idx].is_ascii_whitespace() {
        *idx += 1;
    }
    if *idx >= n {
        return None;
    }

    let start = *idx;
    // Collect a maximal candidate token containing characters that may appear
    // in a floating-point literal. We'll try the longest such slice that
    // successfully parses as f64.
    let mut end = start;
    while end < n {
        let c = bytes[end] as char;
        if "+-0123456789.eE".contains(c) {
            end += 1;
        } else {
            break;
        }
    }

    if start == end {
        return None;
    }

    // Try longest-first shrinking until parse succeeds.
    while end > start {
        let token = &s[start..end];
        if let Ok(v) = token.parse::<f64>() {
            *idx = end;
            return Some(v);
        }
        end -= 1;
    }

    None
}

fn main() {
    // Read full stdin into a string.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, behave like C program that would just exit with 0.
        return;
    }

    let mut i = 0usize;
    let mut mov: f64;
    let mut dir: f64;

    // Initialize position and direction to 0.0 (C code used uninitialized variables;
    // initializing to zero matches the intended behavior of accumulating movements).
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut d: f64 = 0.0;

    loop {
        // Parse first double (scanf's %lf skips leading whitespace).
        match parse_double(&input, &mut i) {
            Some(v) => mov = v,
            None => break, // no more well-formed input
        }

        // Now, scanf had a literal ',' in the format string which must match the next character
        // exactly (scanf does NOT skip whitespace for a literal). Emulate that: the next character
        // must be ','.
        if i >= input.len() {
            break;
        }
        let next_char = input.as_bytes()[i] as char;
        if next_char != ',' {
            break;
        }
        i += 1; // consume the comma

        // Parse second double (scanf's %lf again skips leading whitespace).
        match parse_double(&input, &mut i) {
            Some(v) => dir = v,
            None => break,
        }

        // Loop condition in C: while (scanf(...), mov != 0 || dir != 0) { ... }
        // That means if both are zero, stop without performing updates.
        if mov == 0.0 && dir == 0.0 {
            break;
        }

        // Update position and direction exactly as in the C code.
        let rad = d * PI / 180.0;
        x += rad.sin() * mov;
        y += rad.cos() * mov;
        d += dir;
    }

    // Print as in C: two integers (casts from double to int truncate toward zero),
    // each followed by a newline. We use trunc() then cast to i32 to preserve truncation.
    let xi = x.trunc() as i32;
    let yi = y.trunc() as i32;
    // Print exactly two lines, each ending with a newline.
    println!("{}\n{}", xi, yi);
}
