// translate_sum.rs
//
// Translated from the provided C program per the SPEC.
// Behavior:
//  - Repeatedly reads whitespace-separated decimal integers from stdin.
//  - For each integer `a` read, computes `1 + sum_{i=1..a} i` and writes it as a decimal
//    integer followed by a newline.
//  - If `a <= 0`, the sum is empty and the output is `1`.
//  - Processing stops at EOF (or if a token cannot be parsed as an integer).
//
// Notes on translation choices:
//  - Uses i32 to model the C `int` type (assumes 32-bit `int` as in the SPEC).
//  - The original C code has undefined behavior on signed integer overflow. In Rust,
//    signed overflow in debug builds panics; to avoid introducing panics and to keep
//    deterministic behavior across builds, accumulation uses wrapping arithmetic
//    (i32::wrapping_add). This is a deliberate choice to avoid runtime panics while
//    preserving a well-defined, deterministic result. This deviates from C's UB but
//    is safer and more predictable in Rust.
//  - The parser assumes well-formed input tokens as per the SPEC. If a token is not a
//    valid integer token (per scanf("%d") semantics), the program stops processing.
//    This keeps behavior reasonable while acknowledging the SPEC's note that malformed
//    tokens are unspecified.
//
// No unsafe code is used.

use std::io::{self, Read};

fn next_i32_from_str(s: &str, idx: &mut usize) -> Option<i32> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut i = *idx;

    // Skip leading ASCII whitespace
    while i < n && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    if i >= n {
        *idx = i;
        return None;
    }

    // Optional sign
    let mut sign: i64 = 1;
    if bytes[i] == b'+' {
        i += 1;
    } else if bytes[i] == b'-' {
        sign = -1;
        i += 1;
    }

    // Must have at least one digit after optional sign, otherwise treat as parse failure/stop.
    if i >= n || !bytes[i].is_ascii_digit() {
        return None;
    }

    // Parse digits into i64 to detect range exceedance vs i32.
    let mut val: i64 = 0;
    while i < n && bytes[i].is_ascii_digit() {
        val = val * 10 + (bytes[i] - b'0') as i64;

        // If value would exceed the range of C int (assumed i32 here), treat as parse failure.
        // SPEC states such out-of-range tokens are undefined; we stop processing in that case.
        if sign == 1 && val > i32::MAX as i64 {
            return None;
        }
        // For negative numbers, allow up to absolute value of i32::MIN
        if sign == -1 && val > (i32::MAX as i64) + 1 {
            return None;
        }

        i += 1;
    }

    *idx = i;
    let signed = (val as i64) * sign;
    Some(signed as i32)
}

fn main() -> io::Result<()> {
    // Read entire stdin into a string. The SPEC assumes well-formed input and does not
    // mandate zero dynamic allocation; this approach is simple and matches tokenization
    // semantics adequately for the intended use.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut idx: usize = 0;
    while let Some(a) = next_i32_from_str(&input, &mut idx) {
        // Compute b = sum_{i=1..a} i using an explicit loop (O(a) work as in the C code).
        // Use wrapping_add to avoid Rust panics on signed overflow and to keep deterministic behavior.
        let mut b: i32 = 0;
        if a >= 1 {
            // RangeInclusive<i32> works even when a < 1 (it becomes empty), but we guard above.
            for c in 1..=a {
                b = b.wrapping_add(c);
            }
        }
        let result = b.wrapping_add(1);
        println!("{}", result);
    }

    // Exit with status 0 (implicit on successful return)
    Ok(())
}
