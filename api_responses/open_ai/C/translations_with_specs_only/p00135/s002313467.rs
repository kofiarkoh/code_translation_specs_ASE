// clock_classify.rs
//
// Translates the provided Specification into Rust.
// Behavior notes (choices made to remain faithful to the spec):
// - We parse input similarly to scanf("%d") then n times scanf("%d:%d").
//   That means: whitespace is skipped before each %d; literal ':' must match exactly
//   immediately after the first integer (no intervening whitespace allowed).
//   After the ':' we allow whitespace before the second %d (because %d skips leading whitespace).
// - We do NOT normalize hours/minutes via modulo. Hour and minute integers are used
//   as read to compute angles: hour_angle = 30*h + 0.5*m, minute_angle = 6*m.
//   Only the one-sided normalization described in the spec is applied:
//     if angle > 180.0 -> angle -= 360.0
//     if angle <= -180.0 -> leave as-is (no upward normalization).
// - All comparisons for classification are strict as specified.
// - If parsing of n fails, or an expected "%d:%d" pair cannot be parsed for an iteration,
//   parsing stops and no further classifications are produced (consistent with unspecified behavior).
//
// This program uses safe Rust only.

use std::io::{self, Read};

fn is_ascii_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

// Parse a signed integer from `bytes` starting at `pos`.
// If `skip_ws` is true, leading ASCII whitespace is skipped first (like %d).
// Returns Some((value, new_pos)) on success where new_pos is the index after the last digit.
// Returns None if no integer is found at that position.
fn parse_int(bytes: &[u8], mut pos: usize, skip_ws: bool) -> Option<(i64, usize)> {
    if skip_ws {
        while pos < bytes.len() && is_ascii_whitespace(bytes[pos]) {
            pos += 1;
        }
    }
    if pos >= bytes.len() {
        return None;
    }
    // optional sign
    let mut sign = 1i64;
    if bytes[pos] == b'+' || bytes[pos] == b'-' {
        if bytes[pos] == b'-' {
            sign = -1;
        }
        pos += 1;
        if pos >= bytes.len() {
            // sign without digits => parse failure
            return None;
        }
    }
    let start_digits = pos;
    while pos < bytes.len() && (b'0'..=b'9').contains(&bytes[pos]) {
        pos += 1;
    }
    if start_digits == pos {
        // no digits
        return None;
    }
    // Safe: slice contains only ASCII digits, so it's valid UTF-8
    let s = std::str::from_utf8(&bytes[start_digits..pos]).ok()?;
    // parse as i64; if overflow or other error, fail parsing.
    // This keeps behavior well-defined; spec doesn't require exact overflow semantics.
    let magnitude: i64 = s.parse().ok()?;
    Some((sign * magnitude, pos))
}

fn main() {
    // Read all stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, do nothing (spec allows unspecified behavior); exit.
        return;
    }
    let bytes = input.as_bytes();
    let mut pos = 0usize;

    // Parse n: scanf("%d") semantics => skip leading whitespace
    let (n, newpos) = match parse_int(bytes, pos, true) {
        Some(res) => res,
        None => {
            // If initial integer missing/malformed, behavior unspecified -> produce no output.
            return;
        }
    };
    pos = newpos;

    // For each of n iterations, attempt to parse "%d:%d"
    // Behavior: for each iteration where the pattern successfully parses, produce one line.
    if n <= 0 {
        // nothing to do
        return;
    }
    for _i in 0..n {
        // Before the first %d, scanf("%d:%d") would skip whitespace
        // So skip whitespace before parsing h.
        while pos < bytes.len() && is_ascii_whitespace(bytes[pos]) {
            pos += 1;
        }
        // Parse h: do not allow preceding failure to borrow previous value; if parse fails, stop.
        let (h, after_h) = match parse_int(bytes, pos, false) {
            Some(v) => v,
            None => break,
        };
        pos = after_h;
        // Next character must be a literal ':' (no whitespace skipping between the %d and the literal).
        if pos >= bytes.len() || bytes[pos] != b':' {
            // parse failure for this iteration; stop processing further (unspecified behavior).
            break;
        }
        pos += 1; // consume ':'

        // Now parse m: scanf's %d will skip leading whitespace, so allow that.
        let (m, after_m) = match parse_int(bytes, pos, true) {
            Some(v) => v,
            None => break,
        };
        pos = after_m;

        // Compute angles as described:
        // hour_angle = 30*h + 0.5*m
        // minute_angle = 6*m
        // angle = hour_angle - minute_angle
        // if angle > 180 then angle -= 360
        // if angle <= -180 no upward normalization performed
        let h_f = h as f64;
        let m_f = m as f64;
        let hour_angle = 30.0 * h_f + 0.5 * m_f;
        let minute_angle = 6.0 * m_f;
        let mut angle = hour_angle - minute_angle;
        if angle > 180.0 {
            angle -= 360.0;
        }

        // Classification with strict comparisons per spec:
        // "alert" if angle is strictly between -30 and 30
        // else "warning" if strictly between -90 and 90
        // else "safe"
        let out = if angle > -30.0 && angle < 30.0 {
            "alert"
        } else if angle > -90.0 && angle < 90.0 {
            "warning"
        } else {
            "safe"
        };
        println!("{}", out);
    }
}
