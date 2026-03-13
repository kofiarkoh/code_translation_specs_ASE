// pythag_count.rs
//
// Translated from the provided C program according to the SPEC.
// Notes on differences / rationale:
// - The SPEC is authoritative. The original C used `while (scanf("%d,%d,%d", &a,&b,&c) != EOF)`
//   which has subtle behavior on malformed input (e.g., scanf returning 0 on matching failure
//   can lead to different control flow). The SPEC treats malformed input as "not guaranteed",
//   so this Rust translation stops processing when it cannot successfully parse a full
//   comma-separated triple as described by the SPEC. This is a deliberate, documented choice.
// - To avoid Rust panic-on-overflow in debug builds, integer squaring for comparison is done
//   using i64 temporaries. The SPEC assumes inputs are within the range of C int; using i64
//   preserves intended comparisons without panicking on large-but-valid inputs.
// - The parsing enforces the same comma-placement semantics as `scanf("%d,%d,%d")`:
//   spaces are allowed before each integer (because %d skips leading whitespace), but spaces
//   immediately before a literal comma are NOT allowed. That is: "1, 2,3" is accepted,
//   but "1 ,2,3" is rejected (parsing stops).
//
// The program reads from stdin and writes exactly two lines to stdout:
//  - first line: count of triples where c*c == a*a + b*b
//  - second line: count of triples where a == b
//
// No other output is produced.

use std::io::{self, Read};

fn is_ascii_whitespace(b: u8) -> bool {
    // matches C isspace for ASCII
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b /* vertical tab */ | 0x0c /* form feed */)
}

/// Parse an integer (i32) from `bytes` starting at `pos`.
/// Leading ASCII whitespace is skipped. Returns Some((value, next_pos)) if at least one digit
/// was parsed (with optional leading '+' or '-'); returns None if no digits found (e.g. EOF
/// or non-digit where a number was expected). If the parsed integer does not fit in i32,
/// parsing fails and returns None.
///
/// This function does not allow underscores or other numeral separators.
fn parse_int_i32(bytes: &[u8], mut pos: usize) -> Option<(i32, usize)> {
    let len = bytes.len();
    // Skip leading whitespace
    while pos < len && is_ascii_whitespace(bytes[pos]) {
        pos += 1;
    }
    if pos >= len {
        return None;
    }

    // Optional sign
    let mut sign: i64 = 1;
    if bytes[pos] == b'+' {
        pos += 1;
    } else if bytes[pos] == b'-' {
        sign = -1;
        pos += 1;
    }

    if pos >= len {
        return None; // no digits after sign
    }

    // At least one digit required
    if !(bytes[pos] >= b'0' && bytes[pos] <= b'9') {
        return None;
    }

    let mut acc: i64 = 0;
    while pos < len {
        let b = bytes[pos];
        if b < b'0' || b > b'9' { break; }
        acc = acc * 10 + (i64::from(b - b'0'));
        pos += 1;
        // We don't early-fail on overflow here; we'll check fit into i32 at the end.
    }

    let val_i64 = acc * sign;
    // Ensure it fits into i32 - per SPEC inputs should be in C int range; if not, treat as parse failure.
    if let Ok(v) = i32::try_from(val_i64) {
        Some((v, pos))
    } else {
        None
    }
}

fn main() {
    // Read entire stdin into memory (allowed by SPEC: input is sequential and this is safe for typical usage).
    let mut input = String::new();
    if let Err(_) = io::stdin().read_to_string(&mut input) {
        // If reading stdin fails, per SPEC there's no explicit error reporting; exit quietly.
        // But to follow typical program semantics, we'll just print zeros.
        println!("0");
        println!("0");
        return;
    }
    let bytes = input.as_bytes();
    let mut pos: usize = 0;
    let len = bytes.len();

    let mut pyth_count: usize = 0;
    let mut equal_count: usize = 0;

    loop {
        // Parse first integer (a)
        // Skip leading whitespace first; if EOF, we're done.
        // parse_int_i32 skips leading whitespace itself and returns None on EOF or non-digit.
        let (a, next_pos) = match parse_int_i32(bytes, pos) {
            Some(t) => t,
            None => break, // EOF or cannot parse integer -> stop processing
        };
        pos = next_pos;

        // After first integer, scanf("%d,%d,%d") expects a literal ',' next.
        // That means there must be a comma immediately at current pos (no whitespace allowed before comma).
        if pos >= len {
            // EOF before comma -> stop processing (scanf would return EOF here)
            break;
        }
        if bytes[pos] != b',' {
            // Not a comma in expected position -> treat as malformed and stop (SPEC allows undefined behavior;
            // we choose to stop).
            break;
        }
        pos += 1; // consume comma

        // Parse second integer (b). %d after comma skips leading whitespace.
        let (b, next_pos) = match parse_int_i32(bytes, pos) {
            Some(t) => t,
            None => break,
        };
        pos = next_pos;

        // Expect a comma immediately after the second integer (no whitespace allowed before comma).
        if pos >= len {
            break;
        }
        if bytes[pos] != b',' {
            break;
        }
        pos += 1; // consume comma

        // Parse third integer (c). %d after comma skips leading whitespace.
        let (c, next_pos) = match parse_int_i32(bytes, pos) {
            Some(t) => t,
            None => break,
        };
        pos = next_pos;

        // Successfully parsed a triple (a,b,c). Evaluate predicates.
        // Use i64 temporaries to avoid Rust overflow panic while preserving intended numeric comparison.
        let ai = i64::from(a);
        let bi = i64::from(b);
        let ci = i64::from(c);

        if ci * ci == ai * ai + bi * bi {
            pyth_count += 1;
        }
        if a == b {
            equal_count += 1;
        }

        // Continue: scanf("%d,%d,%d") would allow whitespace between triples because the next %d
        // at the start of the next iteration skips leading whitespace. Our parse_int_i32 call
        // already skips leading whitespace, so just loop again.
        // If pos is at EOF, next parse_int_i32 will return None and we'll break.
    }

    // Print exactly two lines with the counts
    println!("{}", pyth_count);
    println!("{}", equal_count);
}
