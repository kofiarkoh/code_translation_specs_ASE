// turtle.rs
//
// Translation of the provided C program to Rust following the SPEC.
// NOTE: The C code declares x, y, d without initializing them (undefined behavior).
// The SPEC treats the intended behavior as if x, y, d start at 0.0.
// This Rust translation follows the SPEC and initializes x, y, d to 0.0.
//
// Behavior:
// - Reads pairs "mov,dir" from stdin until the sentinel pair 0,0 is read.
// - For each pair (except the terminating 0,0), updates:
//     x += mov * sin(d in degrees)
//     y += mov * cos(d in degrees)
//     d += dir
// - Prints the truncated integer parts of x and y on separate lines.

use std::io::{self, Read};

fn main() {
    // Initialize to 0.0 as per SPEC (to avoid the undefined behavior present in the C code).
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut d: f64 = 0.0;

    // Read entire stdin into a string
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we can't read stdin, behave simply by printing initial position truncated.
        print_truncated(x, y);
        return;
    }

    // Parser: looks for sequences of "<mov>,<dir>" where mov may be followed by optional
    // whitespace before the comma and dir may be preceded/followed by whitespace.
    // Algorithm:
    //  - skip leading whitespace
    //  - find next comma; substring before comma is first number
    //  - after comma, skip whitespace, then take characters up to next whitespace as second number
    // This matches the scanf("%lf,%lf") requirement of a literal comma between numbers.
    let s = input.as_str();
    let mut i: usize = 0;
    let len = s.len();

    while i < len {
        // Skip leading whitespace
        while i < len && s[i..].chars().next().unwrap().is_whitespace() {
            i += s[i..].chars().next().unwrap().len_utf8();
        }
        if i >= len {
            break;
        }

        // Find next comma for first number
        let comma_pos_opt = s[i..].find(',');
        let comma_pos = match comma_pos_opt {
            Some(pos) => i + pos,
            None => break, // no more pairs (malformed / EOF)
        };

        // Extract first number substring and parse
        let first_sub = &s[i..comma_pos].trim();
        if first_sub.is_empty() {
            break; // malformed
        }
        let mov = match first_sub.parse::<f64>() {
            Ok(v) => v,
            Err(_) => break, // malformed
        };

        // Advance past comma
        let mut k = comma_pos + 1;

        // Skip whitespace after comma
        while k < len && s[k..].chars().next().unwrap().is_whitespace() {
            k += s[k..].chars().next().unwrap().len_utf8();
        }
        if k >= len {
            break; // malformed / EOF
        }

        // Find end of second number (next whitespace or end of input)
        let mut l = k;
        while l < len {
            let ch = s[l..].chars().next().unwrap();
            if ch.is_whitespace() {
                break;
            }
            l += ch.len_utf8();
        }
        let second_sub = &s[k..l].trim();
        if second_sub.is_empty() {
            break; // malformed
        }
        let dir = match second_sub.parse::<f64>() {
            Ok(v) => v,
            Err(_) => break, // malformed
        };

        // Advance i to continue parsing from l
        i = l;

        // Check sentinel: do not process the terminating 0,0 pair
        if mov == 0.0 && dir == 0.0 {
            break;
        }

        // Update position: note that d is in degrees
        let rad = d.to_radians();
        x += mov * rad.sin();
        y += mov * rad.cos();

        // Update heading
        d += dir;
    }

    print_truncated(x, y);
}

// Print truncated integer parts of x and y on separate lines.
// Truncation toward zero is achieved by casting via `trunc()` then to i32.
// This mimics C's (int)x behavior for typical finite values.
fn print_truncated(x: f64, y: f64) {
    // Truncate toward zero
    let xi = truncate_to_i32(x);
    let yi = truncate_to_i32(y);
    println!("{}", xi);
    println!("{}", yi);
}

// Convert a f64 to i32 by truncating toward zero.
// If value is NaN, produce 0 (mirroring common C behavior of (int)NaN -> 0 on many platforms).
// If value is out of i32 range, clamp to i32::MIN or i32::MAX to avoid UB.
fn truncate_to_i32(v: f64) -> i32 {
    if v.is_nan() {
        return 0;
    }
    let t = v.trunc();
    if t >= (i32::MAX as f64) {
        i32::MAX
    } else if t <= (i32::MIN as f64) {
        i32::MIN
    } else {
        t as i32
    }
}
