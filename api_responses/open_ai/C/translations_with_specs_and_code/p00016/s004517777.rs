use std::io::{self, Read};

/// This program follows the provided SPEC:
/// - Reads comma-separated pairs of doubles "<mov>,<dir>" from stdin.
/// - For each pair (mov, dir), while the pair is not exactly (0.0, 0.0):
///     - Update the internal direction D by adding dir (degrees).
///     - Move by mov units in the new direction: x += mov * sin(D in radians),
///       y += mov * cos(D in radians).
/// - After termination (when mov == 0.0 && dir == 0.0) print two lines:
///     - First line: (int)x followed by newline
///     - Second line: (int)y followed by newline
///
/// Notes about differences from the provided C code:
/// - The C code used uninitialized x,y,d and performed the move before adding dir.
///   That is undefined behavior in C. Per the SPEC (treat it as definitive), we:
///   * Initialize x = 0.0, y = 0.0, d = 0.0 (start at origin, direction 0).
///   * Update direction first (d += dir), then move (as described above).
/// - The input parsing enforces a literal comma between the two numbers, matching
///   scanf("%lf,%lf") behavior: whitespace is allowed before the first number
///   and after the comma (before the second number), but NOT between the first
///   number and the comma itself.
/// - Casting to integer truncates toward zero by using trunc() then casting to i32,
///   matching C's truncation semantics for typical finite values.
///
/// The program assumes well-formed input per the SPEC (an eventual terminating pair 0,0).
fn main() {
    // Read all stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we can't read stdin, exit (SPEC expects normal execution with proper input).
        return;
    }
    let s = input.as_str();
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut pos = 0_usize;

    // internal state initialized per SPEC (start at origin, direction 0)
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut d: f64 = 0.0;

    // Helper: skip ASCII whitespace
    fn skip_ws(bytes: &[u8], len: usize, pos: &mut usize) {
        while *pos < len {
            let c = bytes[*pos];
            if c == b' ' || c == b'\n' || c == b'\r' || c == b'\t' || c == b'\x0c' || c == b'\x0b' {
                *pos += 1;
            } else {
                break;
            }
        }
    }

    // Parse a floating point number starting at pos.
    // Returns Some((f64, new_pos)) on success where new_pos is the first byte after the number.
    // Accepts patterns similar to: [+|-]? (digits [. digits]? | . digits) ([eE][+|-]?digits)?
    fn parse_f64(bytes: &[u8], len: usize, pos: &mut usize) -> Option<f64> {
        let start = *pos;
        if start >= len {
            return None;
        }

        let mut i = start;

        // optional sign
        if i < len && (bytes[i] == b'+' || bytes[i] == b'-') {
            i += 1;
        }

        let mut digits_before = 0usize;
        while i < len && (bytes[i] as char).is_ascii_digit() {
            i += 1;
            digits_before += 1;
        }

        // fractional part
        let mut digits_after = 0usize;
        if i < len && bytes[i] == b'.' {
            i += 1;
            while i < len && (bytes[i] as char).is_ascii_digit() {
                i += 1;
                digits_after += 1;
            }
        }

        // need at least one digit either before or after decimal point
        if digits_before == 0 && digits_after == 0 {
            return None;
        }

        // exponent part
        if i < len && (bytes[i] == b'e' || bytes[i] == b'E') {
            let exp_pos = i;
            i += 1;
            if i < len && (bytes[i] == b'+' || bytes[i] == b'-') {
                i += 1;
            }
            let mut exp_digits = 0usize;
            while i < len && (bytes[i] as char).is_ascii_digit() {
                i += 1;
                exp_digits += 1;
            }
            if exp_digits == 0 {
                // invalid exponent, treat as parse failure
                return None;
            }
            // valid exponent present
            let _ = exp_pos; // no-op, just illustrating we consumed it
        }

        // substring
        let token = match std::str::from_utf8(&bytes[start..i]) {
            Ok(t) => t,
            Err(_) => return None,
        };

        match token.parse::<f64>() {
            Ok(v) => {
                *pos = i;
                Some(v)
            }
            Err(_) => None,
        }
    }

    loop {
        // skip whitespace before first number (scanf skips leading whitespace for %lf)
        skip_ws(bytes, len, &mut pos);

        // attempt to parse first number (mov)
        let mov = match parse_f64(bytes, len, &mut pos) {
            Some(v) => v,
            None => break, // malformed input or EOF: stop processing (SPEC assumes well-formed input)
        };

        // Immediately expect a literal comma (no intervening whitespace allowed between number and comma)
        if pos >= len || bytes[pos] != b',' {
            // malformed per "%lf,%lf" expectation
            break;
        }
        pos += 1; // consume comma

        // After comma, scanf would skip whitespace before second %lf
        skip_ws(bytes, len, &mut pos);

        // parse second number (dir)
        let dir = match parse_f64(bytes, len, &mut pos) {
            Some(v) => v,
            None => break,
        };

        // Termination condition: exactly (0.0, 0.0)
        if mov == 0.0 && dir == 0.0 {
            break;
        }

        // Per SPEC: update direction first, then move
        d += dir;
        let rad = d.to_radians();
        x += mov * rad.sin();
        y += mov * rad.cos();

        // continue to next pair
    }

    // Cast to int using truncation toward zero (like C's (int)double for typical finite values)
    let xi = x.trunc() as i32;
    let yi = y.trunc() as i32;

    // Print exactly two lines with integer values, each followed by newline
    println!("{}", xi);
    println!("{}", yi);
}
