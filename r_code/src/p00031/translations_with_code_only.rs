// weight.rs
// Translated from Aizu Vol0 0031: Weight (C -> Rust)
// Preserves semantics of the original C program as closely as practical.

use std::io::{self, BufRead, Write};

fn atoi_like(s: &str) -> i32 {
    // A minimal reimplementation of C's atoi:
    // - skip leading whitespace
    // - optional '+' or '-' sign
    // - parse consecutive ASCII digits, stop at first non-digit
    // - if no digits, return 0
    //
    // For overflow we use two's-complement wrapping (wrapping_mul/wrapping_add)
    // to approximate common C implementation behavior rather than panicking.
    let mut chars = s.chars().peekable();
    // skip leading whitespace
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
    // sign
    let mut neg = false;
    if let Some(&c) = chars.peek() {
        if c == '+' {
            chars.next();
        } else if c == '-' {
            neg = true;
            chars.next();
        }
    }
    // parse digits
    let mut acc: i32 = 0;
    let mut any = false;
    while let Some(&c) = chars.peek() {
        if let Some(d) = c.to_digit(10) {
            any = true;
            // accumulate with wrapping to approximate possible C undefined behavior on overflow
            acc = acc.wrapping_mul(10).wrapping_add(d as i32);
            chars.next();
        } else {
            break;
        }
    }
    if !any {
        0
    } else if neg {
        acc.wrapping_neg()
    } else {
        acc
    }
}

fn main() {
    // weights array as in the C code
    let w: [i32; 10] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512];

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for line_res in stdin.lock().lines() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => break, // mirror reading behavior: stop on error/EOF
        };
        let mut x: i32 = atoi_like(&line);

        let mut first = true;
        let mut i: usize = 0;

        while x > 0 {
            if (x & 1) != 0 {
                if first {
                    first = false;
                } else {
                    // print space between numbers
                    let _ = write!(out, " ");
                }
                // Direct indexing as in the C code. If i >= 10 this would be undefined in C;
                // in Rust this will panic. The original C program likewise had undefined
                // behavior for out-of-range indices. We keep the same direct indexing here.
                let _ = write!(out, "{}", w[i]);
            }
            i = i.wrapping_add(1);
            x >>= 1;
        }
        let _ = writeln!(out);
    }
}
