// duplicate_numbers.rs
//
// Translation of the provided C program according to the SPEC.
// Reads pairs of integers from stdin in the form "<number>,<ac>"
// and prints each first-number that appears two or more times along
// with its total occurrence count, one per line, sorted by value.
//
// Parsing semantics intentionally follow the scanf("%d,%d") pattern as described in the SPEC:
// - Leading whitespace before the first %d is allowed.
// - The comma in the input must immediately follow the digits of the first integer (no intervening characters).
// - Whitespace after the comma (before the second %d) is allowed.
// - Each pair is parsed until EOF. We store up to 10000 first-integers (per SPEC limit).

use std::io::{self, Read};
use std::process::exit;

const MAX_PAIRS: usize = 10000;

fn is_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

/// Parse an integer starting at `pos` without skipping leading whitespace.
/// Returns (value as i32, new_pos) on success, or None on failure (e.g., no digits, overflow).
///
/// This function requires that at pos there is either a '+' / '-' sign or a digit.
/// It reads an optional sign then requires at least one digit.
fn parse_int_no_leading_ws(s: &[u8], mut pos: usize) -> Option<(i32, usize)> {
    let len = s.len();
    if pos >= len {
        return None;
    }

    let mut negative = false;
    if s[pos] == b'+' {
        pos += 1;
    } else if s[pos] == b'-' {
        negative = true;
        pos += 1;
    }

    // Must have at least one digit
    if pos >= len || !s[pos].is_ascii_digit() {
        return None;
    }

    // Accumulate in i64 to detect overflow for i32
    let mut acc: i64 = 0;
    while pos < len && s[pos].is_ascii_digit() {
        acc = acc * 10 + (s[pos] - b'0') as i64;
        pos += 1;
        // Early check: if acc exceeds i32::MAX + 1, it's out of range
        if acc > (i32::MAX as i64) + 1 {
            return None;
        }
    }

    if negative {
        acc = -acc;
    }

    // Convert to i32 (SPEC promises inputs fit in C int; we still guard)
    let value = match i32::try_from(acc) {
        Ok(v) => v,
        Err(_) => return None,
    };

    Some((value, pos))
}

/// Parse an integer starting at `pos`, skipping leading whitespace first.
/// Returns (value as i32, new_pos) on success, or None on failure.
fn parse_int_with_ws(s: &[u8], mut pos: usize) -> Option<(i32, usize)> {
    let len = s.len();
    while pos < len && is_whitespace(s[pos]) {
        pos += 1;
    }
    if pos >= len {
        return None;
    }
    parse_int_no_leading_ws(s, pos)
}

fn parse_pairs(input: &str) -> Vec<i32> {
    let bytes = input.as_bytes();
    let mut pos: usize = 0;
    let len = bytes.len();
    let mut numbers: Vec<i32> = Vec::new();

    while pos < len {
        // Skip leading whitespace before first %d
        while pos < len && is_whitespace(bytes[pos]) {
            pos += 1;
        }
        if pos >= len {
            break;
        }

        // At this point, next char should be sign or digit for the first integer.
        // Parse first integer without allowing whitespace between digits and the comma.
        let (first, new_pos) = match parse_int_no_leading_ws(bytes, pos) {
            Some(p) => p,
            None => {
                // Malformed or incomplete; per SPEC input must be well-formed.
                // We stop parsing further pairs on failure (scanf would fail).
                break;
            }
        };
        pos = new_pos;

        // After the digits of the first integer, the next character must be a literal comma.
        if pos >= len {
            // EOF before comma -> incomplete pair -> stop.
            break;
        }
        if bytes[pos] != b',' {
            // Malformed according to "%d,%d" (comma must immediately follow digits) -> stop.
            break;
        }
        pos += 1; // consume comma

        // After comma, whitespace is allowed, then parse the second integer.
        let (_second, new_pos2) = match parse_int_with_ws(bytes, pos) {
            Some(p) => p,
            None => {
                // Malformed or incomplete -> stop.
                break;
            }
        };
        pos = new_pos2;

        // Store first integer if we haven't reached MAX_PAIRS.
        if numbers.len() < MAX_PAIRS {
            numbers.push(first);
        } else {
            // SPEC allows up to 10000 pairs; behavior beyond is undefined in C.
            // To avoid panics in Rust, we simply stop collecting additional pairs.
            break;
        }
    }

    numbers
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    if let Err(_) = io::stdin().read_to_string(&mut input) {
        // On read error, exit non-zero. The original C program did not check read errors,
        // but this is a sensible minimal handling.
        exit(1);
    }

    let mut numbers = parse_pairs(&input);

    // Sort in non-decreasing (ascending) order
    numbers.sort();

    // Scan sorted list and print values that occur two or more times.
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut i = 0usize;
    let n = numbers.len();
    while i < n {
        let v = numbers[i];
        let mut j = i + 1;
        while j < n && numbers[j] == v {
            j += 1;
        }
        let count = j - i;
        if count >= 2 {
            // Print "v count\n" exactly one line per duplicate value
            // Using writeln! to avoid extra spaces/newlines.
            use std::io::Write;
            let _ = writeln!(handle, "{} {}", v, count);
        }
        i = j;
    }

    // Normal termination with exit code 0
}
