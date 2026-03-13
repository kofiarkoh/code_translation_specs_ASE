// translation.rs
//
// Translates the provided specification to Rust.
// Notes on choices:
// - The specification is slightly ambiguous about initialization of internal
//   position (x, y) and direction (d). The most reasonable, narrow
//   interpretation that matches the "logical effect" described is to start
//   at the origin with zero direction. We therefore initialize x = y = d = 0.0.
// - Parsing aims to emulate scanf("%lf,%lf") for a sequence of pairs. We
//   require a comma between the two numbers of a pair (the next non-whitespace
//   character after the first number must be ',').
// - The loop stops when a pair is exactly (0.0, 0.0) (bitwise equality).
// - Movement updates use X += mov * sin(d_deg) and Y += mov * cos(d_deg)
//   as described in the specification (f = sin, g = cos).
// - Casting to integers truncates toward zero (we use f64::trunc() then cast).
// - The implementation assumes well-formed input as per the spec.

use std::f64::consts::PI;
use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // Following the spec's narrow interpretation, on I/O error we simply exit.
        return;
    }
    let s = input.as_str();
    let mut pos = 0usize;
    let len = s.len();

    // Internal state: start at origin with zero direction (chosen as the
    // narrow reasonable interpretation of the specification).
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut d: f64 = 0.0;

    // Helper: skip ASCII whitespace
    let skip_ws = |s: &str, mut i: usize| -> usize {
        let bs = s.as_bytes();
        while i < s.len() {
            match bs[i] {
                b' ' | b'\t' | b'\n' | b'\r' | b'\x0C' | b'\x0B' => i += 1,
                _ => break,
            }
        }
        i
    };

    // Helper: parse a floating token starting at pos. Accept chars in "+-0123456789.eE".
    // Returns (parsed_f64, new_pos) on success, or None if no token or parse failure.
    fn parse_f64_token(s: &str, start: usize) -> Option<(f64, usize)> {
        let bytes = s.as_bytes();
        let mut i = start;
        let n = s.len();

        if i >= n {
            return None;
        }

        // Collect token characters that are valid in a typical decimal float representation.
        // This is a pragmatic approximation of scanf's %lf tokenization for decimal floats.
        let mut end = i;
        while end < n {
            let c = bytes[end];
            let ok = match c {
                b'0'..=b'9' | b'+' | b'-' | b'.' | b'e' | b'E' => true,
                _ => false,
            };
            if ok {
                end += 1;
            } else {
                break;
            }
        }

        if end == i {
            return None;
        }

        // Try to parse the token substring.
        let token = &s[i..end];
        match token.parse::<f64>() {
            Ok(v) => Some((v, end)),
            Err(_) => None,
        }
    }

    // Main loop: parse pairs until we see (0.0, 0.0) or until parsing fails/EOF.
    loop {
        // Skip leading whitespace before first number (scanf would skip)
        pos = skip_ws(s, pos);
        // Parse mov
        let (mov, new_pos) = match parse_f64_token(s, pos) {
            Some(t) => t,
            None => break, // premature EOF / parse failure -> stop processing
        };
        pos = new_pos;

        // After the first number, the specification requires a literal comma between the two numbers.
        // We accept any amount of whitespace, but the next non-whitespace character must be ','.
        pos = skip_ws(s, pos);
        if pos >= len || s.as_bytes()[pos] != b',' {
            // Malformed pair according to the expected "<double>,<double>" pattern.
            // Following the spec's narrow assumption (well-formed input), stop processing.
            break;
        }
        // consume comma
        pos += 1;

        // Skip whitespace then parse dir
        pos = skip_ws(s, pos);
        let (dir, new_pos2) = match parse_f64_token(s, pos) {
            Some(t) => t,
            None => break, // parse failure
        };
        pos = new_pos2;

        // Check termination condition: exact equality to 0.0 for both mov and dir
        if mov == 0.0f64 && dir == 0.0f64 {
            break;
        }

        // Update direction and position. Degrees -> radians conversion.
        d = d + dir;
        let rad = d * (PI / 180.0);
        let sin_r = rad.sin();
        let cos_r = rad.cos();
        // Per specification mapping: X uses sine, Y uses cosine.
        x = x + mov * sin_r;
        y = y + mov * cos_r;
    }

    // Cast to C int semantics: fractional portion discarded toward zero.
    // The spec expects "(int)x" and "(int)y" semantics. We use truncation then cast to i32.
    // This implementation assumes results fit within i32 range as the spec does not define
    // out-of-range behavior.
    let xi = x.trunc() as i32;
    let yi = y.trunc() as i32;

    // Print each integer on its own line.
    println!("{}", xi);
    println!("{}", yi);
}
