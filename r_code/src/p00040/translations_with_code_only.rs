// translation.rs
//
// Direct translation of the provided C program into Rust.
// - Preserves the C program semantics as implemented (searching for an affine
//   decoding that reveals "this" or "that" when applied to a position).
// - Uses byte-wise processing to mirror C's char[] indexing behavior.
// - Emulates affine arithmetic with non-negative modulo result (C used non-negative
//   values here because a and b are in 0..99).
//
// Note: The original C code used fixed-size char[512] and unsafe scanf("%[^\n]")
// which can overflow for very long lines. This Rust version reads lines safely
// from stdin (without artificial truncation) but otherwise preserves the logic.

use std::io::{self, Read, Write};

fn affine_byte(c: u8, a: i32, b: i32) -> u8 {
    // Compute (a * (c - 'a') + b) % 26 + 'a' with a non-negative modulo result.
    // Operate in i32 to mirror C integer arithmetic for these small values.
    let base = ('a' as i32);
    let val = (a * ((c as i32) - base) + b) % 26;
    let val = if val < 0 { val + 26 } else { val };
    (val + base) as u8
}

fn this_check(bytes: &[u8], i: usize, a: i32, b: i32) -> bool {
    // Corresponds to C: 's' == affine(s[0],a,b) && 'i' == affine(s[-1],...) etc.
    // Here `bytes` is the whole line; `i` is the current index (like `line + i`).
    affine_byte(bytes[i], a, b) == b's'
        && affine_byte(bytes[i - 1], a, b) == b'i'
        && affine_byte(bytes[i - 2], a, b) == b'h'
        && affine_byte(bytes[i - 3], a, b) == b't'
}

fn that_check(bytes: &[u8], i: usize, a: i32, b: i32) -> bool {
    affine_byte(bytes[i], a, b) == b't'
        && affine_byte(bytes[i - 1], a, b) == b'a'
        && affine_byte(bytes[i - 2], a, b) == b'h'
        && affine_byte(bytes[i - 3], a, b) == b't'
}

fn search(line: &str, a: i32, b: i32) -> bool {
    let bytes = line.as_bytes();
    let len = bytes.len();
    // C loop: for (i = 3; i < strlen(line); ++i)
    let mut i = 3usize;
    while i < len {
        if bytes[i] == b' ' {
            i += 1;
            continue;
        }
        let y = affine_byte(bytes[i], a, b);
        if (y == b't' && that_check(bytes, i, a, b))
            || (y == b's' && this_check(bytes, i, a, b))
        {
            return true;
        }
        i += 1;
    }
    false
}

fn print_affine_line(line: &str, a: i32, b: i32) {
    let bytes = line.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() + 1);
    for &ch in bytes {
        if ch == b' ' {
            out.push(ch);
        } else {
            out.push(affine_byte(ch, a, b));
        }
    }
    // Write transformed line and a newline, like puts("") in C after printing chars.
    let _ = io::stdout().write_all(&out);
    let _ = io::stdout().write_all(b"\n");
}

fn main() {
    // Read entire stdin and process similarly to the C program's scanf logic.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    // Split into physical lines (like reading with %[^\n]).
    let mut lines_iter = input.lines();

    // Parse n from the first line (trim and take first whitespace-separated token).
    let n_line = lines_iter.next().unwrap_or("");
    let n_token = n_line.trim().split_whitespace().next().unwrap_or("");
    let n: usize = n_token.parse().unwrap_or(0);

    // For each of the next n lines (if there are fewer, treat missing as empty string),
    // try all a,b in 0..100 and print first matching transformed line and exit.
    for _ in 0..n {
        let line = lines_iter.next().unwrap_or("").to_string();
        for a in 0..100 {
            for b in 0..100 {
                if search(&line, a, b) {
                    print_affine_line(&line, a, b);
                    return;
                }
            }
        }
    }
    // If nothing found, program exits normally with no output (like the C program).
}
