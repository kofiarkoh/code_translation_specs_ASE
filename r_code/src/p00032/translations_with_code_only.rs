// translator.rs
// Translated from the given C program.
// Behavior notes:
// - Parses input of the form "%d,%d,%d" repeatedly from stdin until EOF.
// - The commas in the format are matched literally (no skipping of whitespace before commas).
// - Before each %d conversion, leading ASCII whitespace is skipped (to match scanf("%d", ...)).
// - Integer parsing accumulates with wrapping semantics to emulate typical two's-complement
//   32-bit behavior for overflow during arithmetic (as done later with multiplications/additions).
// - The Pythagorean check and additions/multiplications use wrapping arithmetic (i32::wrapping_*)
//   to approximate C's common two's-complement wrap behavior on overflow.
// - If an expected literal comma or an integer cannot be parsed at the correct position, parsing stops.

use std::io::{self, Read};

fn is_ascii_whitespace(b: u8) -> bool {
    // Matches C's isspace for the common ASCII whitespace characters
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0B /*vertical tab*/ | 0x0C /*form feed*/)
}

// Parse a signed decimal integer starting at pos.
// Returns Some((value, new_pos)) if at least one digit was parsed; otherwise None.
// Parsing accumulates with wrapping on i32 to emulate typical 32-bit wraparound.
fn parse_int_wrapping(buf: &[u8], mut pos: usize) -> Option<(i32, usize)> {
    let len = buf.len();
    if pos >= len {
        return None;
    }

    // optional sign
    let mut sign = 1i32;
    match buf[pos] {
        b'+' => { pos += 1; }
        b'-' => { sign = -1; pos += 1; }
        _ => {}
    }

    if pos >= len {
        return None;
    }

    // At least one digit required
    let mut any_digit = false;
    let mut acc: i32 = 0;
    while pos < len {
        let c = buf[pos];
        if c >= b'0' && c <= b'9' {
            any_digit = true;
            let digit = (c - b'0') as i32;
            acc = acc.wrapping_mul(10).wrapping_add(digit);
            pos += 1;
        } else {
            break;
        }
    }

    if !any_digit {
        return None;
    }

    if sign == -1 {
        acc = acc.wrapping_neg();
    }

    Some((acc, pos))
}

fn main() {
    // Read entire stdin
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        // If reading fails, behave as if no input: print zeros per program flow.
        println!("0");
        println!("0");
        return;
    }

    let buf = input.as_slice();
    let mut pos: usize = 0;
    let len = buf.len();

    // counters correspond to C's int tyou=0, hisi=0;
    let mut tyou: i32 = 0;
    let mut hisi: i32 = 0;

    loop {
        // Skip leading whitespace before first %d (scanf("%d" skips leading whitespace)
        while pos < len && is_ascii_whitespace(buf[pos]) {
            pos += 1;
        }
        if pos >= len {
            break; // EOF
        }

        // Parse first integer (a)
        let (a, new_pos) = match parse_int_wrapping(buf, pos) {
            Some(v) => v,
            None => break, // stop on parse failure
        };
        pos = new_pos;

        // Next character must be a literal comma ','
        if pos >= len || buf[pos] != b',' {
            break;
        }
        pos += 1;

        // Before next %d, scanf would skip leading whitespace
        while pos < len && is_ascii_whitespace(buf[pos]) {
            pos += 1;
        }
        if pos >= len {
            break;
        }

        // Parse second integer (b)
        let (b, new_pos) = match parse_int_wrapping(buf, pos) {
            Some(v) => v,
            None => break,
        };
        pos = new_pos;

        // Next character must be a literal comma ','
        if pos >= len || buf[pos] != b',' {
            break;
        }
        pos += 1;

        // Skip leading whitespace before third %d
        while pos < len && is_ascii_whitespace(buf[pos]) {
            pos += 1;
        }
        if pos >= len {
            break;
        }

        // Parse third integer (c)
        let (c, new_pos) = match parse_int_wrapping(buf, pos) {
            Some(v) => v,
            None => break,
        };
        pos = new_pos;

        // All three parsed successfully: perform checks.
        // Use wrapping arithmetic to emulate typical 32-bit wraparound behavior.
        let cc = c.wrapping_mul(c);
        let aa = a.wrapping_mul(a);
        let bb = b.wrapping_mul(b);
        if cc == aa.wrapping_add(bb) {
            tyou = tyou.wrapping_add(1);
        }
        if a == b {
            hisi = hisi.wrapping_add(1);
        }

        // Continue parsing next triple; do not skip extra separators/newlines here
        // because scanf would continue from current position.
    }

    // Print results in the exact format "%d\n%d\n"
    // Using println! produces the same newline-terminated output.
    println!("{}", tyou);
    println!("{}", hisi);
}
