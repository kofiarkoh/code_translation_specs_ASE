// translator.rs
//
// Translates the given Specification into Rust code.
// Behavior notes (kept faithful to the Specification):
// - Input is consumed similarly to scanf("%d", &s):
//   * Leading ASCII whitespace is skipped.
//   * If next character is '+' or '-' it is accepted only if followed by at least one digit.
//   * If next non-whitespace character is not sign or digit, the "matching failure" case occurs:
//     the token is NOT consumed and the caller's loop continues. The original C program's loop
//     checked scanf(...) != EOF and would therefore repeatedly see the same non-matching token,
//     typically causing an infinite loop. This program reproduces that observable behavior:
//     on a non-integer token the scanner returns NoMatch and does not advance the input position.
// - Parsed integers are treated as C int; we use i32 here (narrowest reasonable choice).
// - All arithmetic for the aggregate is performed with wrapping (two's complement) operations
//   to emulate possible C overflow/wraparound behavior as closely as reasonable in Rust.
// - For each successfully parsed integer s we compute:
//     ans = sum_{a=0..A_max} sum_{b=0..B_max(a)} (501 - |500 - (s - a - b)|)
//   with A_max = min(1000, s) and B_max(a) = min(1000, s - a).
// - For s < 0 the sums are empty and ans = 0.
// - On EOF the program exits with status 0.

use std::io::{self, Read, Write};

enum ScanResult {
    Matched(i32),
    NoMatch,
    EOF,
}

fn is_ascii_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0B /* vertical tab */ | 0x0C /* form feed */)
}

fn is_ascii_digit(b: u8) -> bool {
    b'0' <= b && b <= b'9'
}

/// Scan function that mimics scanf("%d", &s) behavior on ASCII bytes read from `input`.
/// It uses `pos` as the current index into `input`. On success it advances `pos` past the
/// matched characters and returns Matched(s). On matching failure it DOES NOT advance `pos`
/// and returns NoMatch (this reproduces the scanf behavior where the offending token is left
/// in the stream). On end-of-input it returns EOF.
fn scan_next_int(input: &[u8], pos: &mut usize) -> ScanResult {
    let len = input.len();

    // Skip leading ASCII whitespace like scanf does for "%d".
    while *pos < len && is_ascii_whitespace(input[*pos]) {
        *pos += 1;
    }

    if *pos >= len {
        return ScanResult::EOF;
    }

    // Peek next char
    let start = *pos;
    let first = input[start];

    // Accept optional sign
    if first == b'+' || first == b'-' {
        // Need at least one digit after sign to match; otherwise it's a matching failure
        let next_pos = start + 1;
        if next_pos >= len || !is_ascii_digit(input[next_pos]) {
            // Do NOT consume the sign (matching failure)
            return ScanResult::NoMatch;
        }
        // There is at least one digit after sign. Consume sign and proceed to read digits.
        let mut idx = next_pos + 1;
        while idx < len && is_ascii_digit(input[idx]) {
            idx += 1;
        }
        // Parse substring start..idx as i32 decimal
        let s_slice = &input[start..idx];
        match std::str::from_utf8(s_slice).and_then(|st| st.parse::<i32>().map_err(|e| e.into())) {
            Ok(v) => {
                *pos = idx;
                ScanResult::Matched(v)
            }
            Err(_) => {
                // Parsing failed (e.g., out of range). The specification assumes tokens are valid ints,
                // but to stay deterministic we treat parse failure as NoMatch without consuming.
                // This mirrors the narrow interpretation: do not invent extra handling.
                ScanResult::NoMatch
            }
        }
    } else if is_ascii_digit(first) {
        // Read digits
        let mut idx = start + 1;
        while idx < len && is_ascii_digit(input[idx]) {
            idx += 1;
        }
        let s_slice = &input[start..idx];
        match std::str::from_utf8(s_slice).and_then(|st| st.parse::<i32>().map_err(|e| e.into())) {
            Ok(v) => {
                *pos = idx;
                ScanResult::Matched(v)
            }
            Err(_) => {
                // As above, treat parse failure as NoMatch and do not consume.
                ScanResult::NoMatch
            }
        }
    } else {
        // First non-whitespace char is not sign or digit: matching failure, do not consume.
        ScanResult::NoMatch
    }
}

fn main() {
    // Read entire stdin into memory. This simplifies implementing exact scanf-like semantics,
    // in particular leaving non-matching tokens in the unread portion of the input.
    let mut input_bytes = Vec::new();
    // If reading stdin errors, treat it like EOF and exit with 0 as per normal termination.
    let _ = io::stdin().read_to_end(&mut input_bytes);

    let mut pos: usize = 0;
    let mut stdout = io::stdout();
    let mut out_lock = stdout.lock();

    loop {
        match scan_next_int(&input_bytes, &mut pos) {
            ScanResult::EOF => {
                // Normal EOF termination, exit with status 0.
                return;
            }
            ScanResult::NoMatch => {
                // Matching failure: mimic original program's behavior where scanf returns 0,
                // leaving the token in the stream. The original program's loop condition
                // checked scanf(...) != EOF, so it would repeatedly attempt to read and
                // repeatedly fail, typically spinning. We reproduce that observable behavior:
                // simply continue the loop without advancing pos. This will normally loop forever.
                continue;
            }
            ScanResult::Matched(s) => {
                // Compute ans according to specification.
                // Use wrapping arithmetic on i32 to emulate C int overflow/wraparound behavior.
                let ans: i32 = compute_ans_wrapping(s);
                // Write ans followed by newline, exactly one line per successfully parsed integer.
                // Ignore write errors (spec: no explicit handling).
                let _ = writeln!(out_lock, "{}", ans);
            }
        }
    }
}

/// Compute the aggregate:
/// ans = sum_{a=0..A_max} sum_{b=0..B_max(a)} (501 - |500 - (s - a - b)|),
/// with A_max = min(1000, s) and B_max(a) = min(1000, s - a).
/// For s < 0 the sum is empty and ans = 0.
///
/// All arithmetic uses wrapping operations on i32 to emulate C int overflow/wraparound.
fn compute_ans_wrapping(s: i32) -> i32 {
    if s < 0 {
        return 0;
    }

    // A_max = min(1000, s)
    let a_max = if s > 1000 { 1000 } else { s } as usize;

    let mut ans: i32 = 0;

    for a_usize in 0..=a_max {
        let a_i = a_usize as i32;
        // s - a is guaranteed >= 0 because a <= A_max <= s
        let rem = s - a_i;
        // B_max = min(1000, rem)
        let b_max = if rem > 1000 { 1000 } else { rem } as usize;

        for b_usize in 0..=b_max {
            let b_i = b_usize as i32;
            // c = s - a - b ; use wrapping_sub to emulate C behavior should overflow occur
            let c = s.wrapping_sub(a_i).wrapping_sub(b_i);
            // d = 500 - c
            let d = 500i32.wrapping_sub(c);
            // abs_d with wrapping negation for negative numbers
            let abs_d = if d < 0 { d.wrapping_neg() } else { d };
            let term = 501i32.wrapping_sub(abs_d);
            ans = ans.wrapping_add(term);
        }
    }

    ans
}
