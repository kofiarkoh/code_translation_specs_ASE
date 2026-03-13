use std::io::{self, BufRead, Write};

/// Parse a string according to "C atoi" semantics as described in the specification:
/// - skip leading ASCII whitespace
/// - optional '+' or '-' sign
/// - parse consecutive ASCII digits into a signed integer
/// - if no digits are found, return 0
/// Note: overflow behavior is unspecified in the spec; we use i64 and do not attempt to emulate C overflow.
fn parse_atoi_c_style(s: &str) -> i64 {
    let bytes = s.as_bytes();
    let mut i = 0;
    // skip leading ASCII whitespace
    while i < bytes.len() {
        match bytes[i] {
            b' ' | b'\t' | b'\n' | b'\r' | 0x0C | 0x0B => i += 1,
            _ => break,
        }
    }

    if i >= bytes.len() {
        return 0;
    }

    let mut neg = false;
    match bytes[i] {
        b'+' => { i += 1; }
        b'-' => { neg = true; i += 1; }
        _ => {}
    }

    let mut any = false;
    let mut val: i64 = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b'0' <= b && b <= b'9' {
            any = true;
            val = val * 10 + (b - b'0') as i64;
            i += 1;
        } else {
            break;
        }
    }

    if !any {
        0
    } else if neg {
        -val
    } else {
        val
    }
}

fn main() {
    // fixed weight table implied by the specification: indices 0..9 => weights 1..512
    // We'll generate weights as (1 << i) for i in 0..10.
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    for line_res in stdin.lock().lines() {
        // Per spec: read until EOF; each line is independent.
        let line = match line_res {
            Ok(l) => l,
            Err(_) => {
                // On I/O error while reading a line, behavior is not specified; treat as EOF/stop.
                break;
            }
        };

        let x = parse_atoi_c_style(&line);

        // Per spec: negative integers result in no weights being printed (blank line).
        // x == 0 => also blank line.
        if x <= 0 {
            let _ = out.write_all(b"\n");
            continue;
        }

        // For safety and to adhere to the specification's supported domain (bits 0..9),
        // only consider the lowest 10 bits (weights 1..512).
        // The specification states behavior for higher bits is undefined; to avoid UB we ignore them.
        let mut first = true;
        for i in 0..10 {
            if ((x >> i) & 1) != 0 {
                if !first {
                    let _ = out.write_all(b" ");
                }
                first = false;
                // weight = 1 << i, fits in i64 for i in 0..9
                let weight = 1i64 << i;
                let _ = write!(out, "{}", weight);
            }
        }
        let _ = out.write_all(b"\n");
    }
}
