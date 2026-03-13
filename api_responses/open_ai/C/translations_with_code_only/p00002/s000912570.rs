// translated from the provided C program.
// Behavior notes:
// - We emulate C's typical 32-bit `int` semantics by using i32 and wrapping on addition.
// - Input is scanned similarly to `scanf("%d %d")`: we skip whitespace, accept an optional '+'/'-'
//   and then read consecutive digits. If there are no digits where a conversion is expected, we
//   stop processing further input (this avoids infinite-loop behavior on malformed input).
// - For very large integer literals we parse into i128 and then cast to i32 (which wraps) to
//   mimic typical C behavior when assigning out-of-range values to a 32-bit `int`.
// - Output matches the C program: for each pair read, print the length (strlen) of the decimal
//   string produced by sprintf("%d", a+b), followed by a newline.
use std::io::{self, Read};

fn read_c_int(input: &str, pos: &mut usize) -> Option<i128> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    // Skip leading whitespace (like scanf)
    while *pos < len && bytes[*pos].is_ascii_whitespace() {
        *pos += 1;
    }
    if *pos >= len {
        return None; // EOF
    }

    // Optional sign
    let mut sign: i128 = 1;
    if bytes[*pos] == b'+' {
        *pos += 1;
    } else if bytes[*pos] == b'-' {
        sign = -1;
        *pos += 1;
    }

    // Read consecutive digits
    let start = *pos;
    let mut value: i128 = 0;
    while *pos < len && bytes[*pos].is_ascii_digit() {
        // accumulate in i128 to avoid early overflow on huge literals
        value = value * 10 + (bytes[*pos] - b'0') as i128;
        *pos += 1;
    }

    if *pos == start {
        // No digits were found where a conversion was expected.
        // The real C code would cause scanf to return 0 and (in the provided loop)
        // would likely loop forever. Here we stop processing further input.
        return None;
    }

    Some(sign * value)
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we cannot read stdin, behave as if EOF (no output).
        return;
    }

    let mut pos: usize = 0;
    loop {
        // Attempt to read two integers like scanf("%d %d", &a, &b)
        let a_opt = read_c_int(&input, &mut pos);
        let b_opt = read_c_int(&input, &mut pos);

        // If either conversion failed or EOF, stop (mirrors typical use with well-formed input).
        let (a_val, b_val) = match (a_opt, b_opt) {
            (Some(a), Some(b)) => (a, b),
            _ => break,
        };

        // Emulate C `int` semantics: cast to i32 (wrap) and use wrapping_add to model two's-complement wrap.
        let a_i32 = a_val as i32;
        let b_i32 = b_val as i32;
        let sum = a_i32.wrapping_add(b_i32);

        // sprintf(buf, "%d", sum) would produce the decimal representation including sign; strlen of that:
        let s = sum.to_string();
        println!("{}", s.len());
    }
}
