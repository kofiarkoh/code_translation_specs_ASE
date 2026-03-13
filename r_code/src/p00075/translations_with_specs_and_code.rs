// bmi_filter.rs
//
// Translated from the provided C program and SPEC.
// Behavior: Read comma-separated triples (int,double,double) from stdin,
// compute BMI = weight / (height * height), and print the id (one per line)
// for records with BMI >= 25. Exit with status 0.
//
// Notes on parsing:
// - This implementation follows the SPEC for intended behavior on well-formed
//   input. It attempts to mimic scanf("%d,%lf,%lf") semantics for well-formed
//   records:
//     * Leading whitespace before the integer is skipped.
//     * The comma literals must appear exactly where required (no whitespace
//       is allowed between the integer and the following comma, nor between
//       the first double and its following comma). Whitespace is allowed
//       after a comma before a numeric conversion because numeric conversions
//       skip leading whitespace.
//   * For malformed or partial input this program stops processing further
//     records (safe behavior) rather than reproducing the undefined behavior
//     of the original C program on malformed input.
//
// No unsafe code is used.

use std::io::{self, Read};

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

fn skip_whitespace(buf: &[u8], mut pos: usize) -> usize {
    let n = buf.len();
    while pos < n {
        let c = buf[pos];
        if c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' || c == 0x0C || c == 0x0B
        {
            pos += 1;
        } else {
            break;
        }
    }
    pos
}

/// Parse an integer starting at pos. Leading whitespace must be skipped by caller.
/// Returns (value, new_pos) on success, or None on failure.
fn parse_int(buf: &[u8], pos: usize) -> Option<(i32, usize)> {
    let n = buf.len();
    if pos >= n {
        return None;
    }
    let mut i = pos;
    // optional sign
    if buf[i] == b'+' || buf[i] == b'-' {
        i += 1;
        if i >= n {
            return None;
        }
    }
    // must have at least one digit
    let start_digits = i;
    while i < n && is_digit(buf[i]) {
        i += 1;
    }
    if i == start_digits {
        return None;
    }
    // parse substring
    let s = std::str::from_utf8(&buf[pos..i]).ok()?;
    // parse into i32 (matching C int assumption)
    match s.parse::<i32>() {
        Ok(v) => Some((v, i)),
        Err(_) => None,
    }
}

/// Parse a floating-point number (f64) starting at pos. Leading whitespace is allowed
/// (scanf "%lf" skips leading whitespace), so caller may or may not skip it.
/// Returns (value, new_pos) on success, or None on failure.
fn parse_float(buf: &[u8], pos: usize) -> Option<(f64, usize)> {
    let n = buf.len();
    if pos >= n {
        return None;
    }
    let mut i = pos;
    // skip leading whitespace (scanf behavior for %lf)
    i = skip_whitespace(buf, i);
    if i >= n {
        return None;
    }
    let start = i;

    // optional sign
    if buf[i] == b'+' || buf[i] == b'-' {
        i += 1;
        if i >= n {
            return None;
        }
    }

    let mut has_digits = false;

    // integer part digits
    while i < n && is_digit(buf[i]) {
        has_digits = true;
        i += 1;
    }

    // fractional part
    if i < n && buf[i] == b'.' {
        i += 1;
        while i < n && is_digit(buf[i]) {
            has_digits = true;
            i += 1;
        }
    }

    if !has_digits {
        return None;
    }

    // exponent part
    if i < n && (buf[i] == b'e' || buf[i] == b'E') {
        let mut j = i + 1;
        if j < n && (buf[j] == b'+' || buf[j] == b'-') {
            j += 1;
        }
        let exp_start = j;
        while j < n && is_digit(buf[j]) {
            j += 1;
        }
        if j == exp_start {
            return None; // no digits in exponent
        }
        i = j;
    }

    let s = std::str::from_utf8(&buf[start..i]).ok()?;
    match s.parse::<f64>() {
        Ok(v) => Some((v, i)),
        Err(_) => None,
    }
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit with 0 as the original program does not report errors.
        return;
    }
    let buf = input.as_bytes();
    let mut pos = 0usize;
    let n = buf.len();

    loop {
        // Skip leading whitespace before integer (scanf does this for %d)
        pos = skip_whitespace(buf, pos);
        if pos >= n {
            break;
        }

        // Parse integer id
        let (id, new_pos) = match parse_int(buf, pos) {
            Some(t) => t,
            None => break, // stop on parse failure
        };
        pos = new_pos;

        // Next character must be a comma (literal in format)
        if pos >= n || buf[pos] != b',' {
            break;
        }
        pos += 1;

        // Parse first double (weight). Leading whitespace allowed.
        let (w, new_pos) = match parse_float(buf, pos) {
            Some(t) => t,
            None => break,
        };
        pos = new_pos;

        // Next character must be a comma
        if pos >= n || buf[pos] != b',' {
            break;
        }
        pos += 1;

        // Parse second double (height). Leading whitespace allowed.
        let (tval, new_pos) = match parse_float(buf, pos) {
            Some(t) => t,
            None => break,
        };
        pos = new_pos;

        // Compute BMI and output id if >= 25
        let bmi = w / (tval * tval);
        if bmi >= 25.0 {
            // Print id followed by newline, and nothing else
            println!("{}", id);
        }

        // Continue parsing from current pos (do not skip additional characters here;
        // the next loop will skip leading whitespace before the next integer).
    }
}
