// turtle.rs
//
// Translation of the given Specification into Rust.
// Notes:
// - The original C specification described uninitialized variables x,y,d.
//   To preserve the intended/assumed behavior (see specification), we
//   initialize x, y, d to 0.0 so the accumulation is well-defined.
// - Input format: a sequence of pairs "mov,dir" (floating-point), with a
//   literal comma separating the two numbers. Pairs are read in order until
//   a sentinel pair 0,0 is seen; that pair is not applied and causes termination.
// - We accept optional whitespace around tokens; the comma must be present
//   (we look for the comma between the two numbers). This is a narrow,
//   reasonable interpretation consistent with the spec's requirement of a literal comma.
// - Final printed outputs are the integer conversions of final x and y,
//   truncated toward zero (as in C's (int)x). We assume results fit in i32
//   per the specification's output schema.

use std::f64::consts::PI;
use std::io::{self, Read};

fn is_num_char(c: char) -> bool {
    // Characters that may appear in a floating-point literal (basic set).
    // We accept digits, sign, decimal point, and exponent letters.
    matches!(c, '0'..='9' | '+' | '-' | '.' | 'e' | 'E')
}

fn skip_whitespace(s: &str, mut pos: usize) -> usize {
    let bytes = s.as_bytes();
    let n = bytes.len();
    while pos < n && (bytes[pos] as char).is_whitespace() {
        pos += 1;
    }
    pos
}

fn parse_number(s: &str, pos: &mut usize) -> Option<f64> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    *pos = skip_whitespace(s, *pos);
    if *pos >= n {
        return None;
    }
    let start = *pos;
    // Collect characters that belong to a numeric token.
    while *pos < n {
        let c = bytes[*pos] as char;
        if is_num_char(c) {
            *pos += 1;
        } else {
            break;
        }
    }
    if start == *pos {
        // No numeric characters found.
        return None;
    }
    let token = &s[start..*pos];
    // Attempt to parse the token as f64.
    match token.parse::<f64>() {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

fn main() {
    // Initialize state to intended values (avoid C undefined behavior).
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut d: f64 = 0.0; // heading in degrees

    // Read all stdin into a string.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we can't read stdin, exit printing current state (which is zeroed).
        // This is a conservative, safe choice.
    }

    let mut pos: usize = 0;
    let len = input.len();

    loop {
        // Parse first number (mov)
        let mov_opt = parse_number(&input, &mut pos);
        if mov_opt.is_none() {
            // Unable to parse further; according to the specification,
            // malformed input leads to undefined behavior. We stop parsing here.
            break;
        }
        let mov = mov_opt.unwrap();

        // After first number, skip whitespace then require a comma literal.
        pos = skip_whitespace(&input, pos);
        if pos >= len {
            // No comma available; malformed input -> stop.
            break;
        }
        // The next character must be a comma separating the two numbers.
        let c = input.as_bytes()[pos] as char;
        if c != ',' {
            // Malformed (missing comma) -> stop.
            break;
        }
        pos += 1; // consume comma

        // Parse second number (dir)
        let dir_opt = parse_number(&input, &mut pos);
        if dir_opt.is_none() {
            // Malformed second number -> stop.
            break;
        }
        let dir = dir_opt.unwrap();

        // Termination condition: when both mov and dir are exactly 0.0
        if mov == 0.0 && dir == 0.0 {
            break; // sentinel read; do not apply this pair
        }

        // Apply movement using the current heading d (in degrees).
        let rad = d * (PI / 180.0);
        x += mov * rad.sin();
        y += mov * rad.cos();
        // Then rotate heading by dir
        d += dir;

        // Continue to next pair
        // (loop will try to parse next mov,dir)
    }

    // Convert final x,y to integers by truncation toward zero (as C's (int)x).
    // The specification assumes values fit into 32-bit signed int for output.
    // We handle NaN/out-of-range conservatively by clamping to i32 bounds,
    // which is a reasonable, safe behavior when the spec's preconditions are met.
    let to_trunc_i32 = |v: f64| -> i32 {
        if v.is_nan() {
            0
        } else {
            let t = v.trunc();
            if t >= (i32::MAX as f64) {
                i32::MAX
            } else if t <= (i32::MIN as f64) {
                i32::MIN
            } else {
                t as i32
            }
        }
    };

    let xi = to_trunc_i32(x);
    let yi = to_trunc_i32(y);

    // Print results: two lines, first (int)x then (int)y
    println!("{}", xi);
    println!("{}", yi);
}
