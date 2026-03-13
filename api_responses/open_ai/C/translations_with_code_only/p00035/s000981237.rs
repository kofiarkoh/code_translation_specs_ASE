// translated from the provided C program
// Preserves scanf("%lf,%lf,...") semantics with respect to skipping leading
// whitespace before each %lf, requiring literal commas (with no intervening
// whitespace), and using IEEE f64 arithmetic for comparisons.

use std::io::{self, Read};

fn gaiseki(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (x0 - x1) * (y0 - y1) - (x0 - x2) * (y0 - y2)
}

// Parse one f64 from s starting at idx, following the behaviour of scanf("%lf", ...)
// - skips leading ASCII whitespace
// - consumes an optional sign, digits and optional fractional part and optional exponent
// - only accepts exponent if it is followed by at least one digit
// - if no digits around decimal point, accepts "inf"/"infinity" and "nan" (case-insensitive)
// Returns Some((value, new_index)) where new_index is the first byte index after the
// parsed floating text. Returns None if there is no non-whitespace input remaining.
fn parse_float(s: &str, mut idx: usize) -> Option<(f64, usize)> {
    let bytes = s.as_bytes();
    let n = bytes.len();

    // skip leading ASCII whitespace (scanf %lf skips leading whitespace)
    while idx < n && bytes[idx].is_ascii_whitespace() {
        idx += 1;
    }
    if idx >= n {
        return None; // EOF / nothing to read
    }

    let start = idx;

    // optional sign
    if bytes[idx] == b'+' || bytes[idx] == b'-' {
        idx += 1;
        if idx >= n {
            // sign alone, treat as no numeric content (will attempt special tokens below)
        }
    }

    let mut pos = idx;
    let mut digits_count = 0usize;

    // integer part digits
    while pos < n && bytes[pos].is_ascii_digit() {
        pos += 1;
        digits_count += 1;
    }

    // fractional part
    if pos < n && bytes[pos] == b'.' {
        pos += 1;
        while pos < n && bytes[pos].is_ascii_digit() {
            pos += 1;
            digits_count += 1;
        }
    }

    // try exponent only if there was at least one digit (so "1e+3" works, but "e10" doesn't)
    if digits_count > 0 && pos < n && (bytes[pos] == b'e' || bytes[pos] == b'E') {
        let epos = pos + 1;
        let mut j = epos;
        if j < n && (bytes[j] == b'+' || bytes[j] == b'-') {
            j += 1;
        }
        let mut exp_digits = 0usize;
        while j < n && bytes[j].is_ascii_digit() {
            j += 1;
            exp_digits += 1;
        }
        if exp_digits > 0 {
            pos = j; // include exponent
        }
        // else: do not include the 'e' if it's not followed by digits
    }

    if digits_count > 0 {
        // we have a numeric-like substring from start..pos
        let substr = &s[start..pos];
        match substr.parse::<f64>() {
            Ok(v) => return Some((v, pos)),
            Err(_) => {
                // parsing failed unexpectedly; return None to signal failure
                return None;
            }
        }
    }

    // digits_count == 0: attempt to parse special tokens "inf"/"infinity" or "nan"
    // allow optional sign before them
    let mut sign = 1.0f64;
    let mut p = start;
    if p < n && bytes[p] == b'+' {
        p += 1;
    } else if p < n && bytes[p] == b'-' {
        sign = -1.0;
        p += 1;
    }
    if p >= n {
        return None; // nothing after sign
    }

    // helper to check case-insensitive prefix match at position p
    let rem = &s[p..];
    let rem_lower = rem.as_bytes().to_ascii_lowercase();
    if rem_lower.starts_with(b"inf") {
        // prefer "infinity" if present
        if rem_lower.starts_with(b"infinity") {
            let end = p + "infinity".len();
            return Some((sign * f64::INFINITY, end));
        } else {
            let end = p + "inf".len();
            return Some((sign * f64::INFINITY, end));
        }
    } else if rem_lower.starts_with(b"nan") {
        let end = p + "nan".len();
        // sign * NAN is still NAN; keeps semantics similar to parsing "-nan"
        return Some((sign * f64::NAN, end));
    }

    // nothing recognized
    None
}

fn main() {
    // Read entire stdin into a String
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let bytes = input.as_bytes();
    let n = bytes.len();
    let mut idx = 0usize;

    'outer: loop {
        // Attempt to parse eight numbers with literal commas between them:
        // pattern: %lf, %lf, %lf, %lf, %lf, %lf, %lf, %lf
        let mut nums = [0f64; 8];
        let mut ok = true;

        for k in 0..8 {
            // parse next float (which skips leading whitespace)
            match parse_float(&input, idx) {
                Some((v, new_idx)) => {
                    nums[k] = v;
                    idx = new_idx;
                    // for the first seven numbers, require an immediate comma (no whitespace)
                    if k < 7 {
                        if idx < n && bytes[idx] == b',' {
                            idx += 1; // consume the comma and continue
                        } else {
                            // scanf would fail to match here (literal comma mismatch)
                            ok = false;
                            break;
                        }
                    }
                    // for k == 7, do not require a comma
                }
                None => {
                    // If no input remaining (EOF reached when skipping whitespace), terminate loop
                    // We determine this by checking whether idx advanced to EOF by skipping whitespace.
                    // parse_float returns None for both EOF and invalid tokens; to reasonably mimic
                    // the original program's behavior across typical inputs, stop on None.
                    ok = false;
                    break;
                }
            }
        }

        if !ok {
            // If we've reached end-of-input (skip whitespace leads to EOF), break the outer loop.
            // Otherwise, the C program's scanf would have returned a value != EOF for malformed
            // input (possibly 0), causing an infinite loop. To avoid that, we stop here as the
            // typical judging inputs are well-formed CSV-like numbers.
            // Determine if remaining input is only whitespace -> then EOF
            let mut t = idx;
            while t < n && bytes[t].is_ascii_whitespace() {
                t += 1;
            }
            if idx >= n || t >= n {
                break 'outer;
            } else {
                // malformed input remaining: stop processing further to avoid infinite loop
                break 'outer;
            }
        }

        // At this point nums[0..8] are populated as f64
        let x0 = nums[0];
        let y0 = nums[1];
        let x1 = nums[2];
        let y1 = nums[3];
        let x2 = nums[4];
        let y2 = nums[5];
        let x3 = nums[6];
        let y3 = nums[7];

        // Compare using >= 0.0, preserving IEEE semantics (NaN comparisons yield false)
        if gaiseki(x0, y0, x1, y1, x3, y3) >= 0.0
            && gaiseki(x1, y1, x2, y2, x3, y3) >= 0.0
            && gaiseki(x2, y2, x0, y0, x3, y3) >= 0.0
        {
            // preserve exact output formatting
            println!("YES");
        } else {
            println!("NO");
        }

        // Continue parsing from current idx; loop will attempt to parse next 8 numbers
        // until EOF or malformed input
        if idx >= n {
            break 'outer;
        }
    }
}
