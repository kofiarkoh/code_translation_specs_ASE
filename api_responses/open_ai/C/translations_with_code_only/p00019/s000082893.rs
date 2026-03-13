// factorial_wrap.rs
//
// Translated from the provided C program.
// Behavior notes:
// - The original C program reads an `int` with scanf("%d", &i) and computes:
//     j = 1; for (; i > 0; i--) j *= i;
//   then prints j with printf("%d\n", j).
// - To emulate typical C `int` arithmetic (32-bit two's-complement wraparound),
//   this Rust translation uses i32 and wrapping arithmetic (wrapping_mul, wrapping_sub).
// - Scanning of the integer is implemented to mimic scanf("%d") for decimal input:
//   it skips leading ASCII whitespace, accepts an optional '+' or '-' sign, then
//   consumes consecutive ASCII digits. The parsed value is computed with
//   wrapping arithmetic so very large digit sequences wrap as they would on a
//   two's-complement machine. If no integer is found, we fall back to 0 (the C
//   code's behavior in that case is undefined; here we choose a deterministic fallback).
//
// This program preserves stdout formatting: a single decimal number followed by a newline.

use std::io::{self, Read};

fn parse_c_decimal_as_i32(input: &str) -> Option<i32> {
    let mut chars = input.chars().peekable();

    // Skip leading ASCII whitespace (scanf("%d") skips whitespace)
    while let Some(&c) = chars.peek() {
        if c.is_ascii_whitespace() {
            chars.next();
        } else {
            break;
        }
    }

    // Optional sign
    let negative = match chars.peek() {
        Some('+') => {
            chars.next();
            false
        }
        Some('-') => {
            chars.next();
            true
        }
        _ => false,
    };

    // At least one digit required to succeed
    let mut any_digit = false;
    let mut value: i32 = 0;

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            any_digit = true;
            let d = (c as u8 - b'0') as i32;
            // emulate C-like wraparound on overflow
            value = value.wrapping_mul(10).wrapping_add(d);
            chars.next();
        } else {
            break;
        }
    }

    if !any_digit {
        None
    } else if negative {
        Some(value.wrapping_neg())
    } else {
        Some(value)
    }
}

fn main() {
    // Read entire stdin (scanf("%d") would consume leading whitespace and digits only,
    // but reading whole input and parsing first token is sufficient for our purposes).
    let mut s = String::new();
    // In the C code scanf's return value isn't checked; we mirror that lax behavior and
    // proceed with a deterministic fallback if parsing fails.
    let _ = io::stdin().read_to_string(&mut s);

    let mut i: i32 = parse_c_decimal_as_i32(&s).unwrap_or(0);

    let mut j: i32 = 1;

    // for (; i > 0; i--) { j *= i; }
    while i > 0 {
        j = j.wrapping_mul(i);
        // decrement with wrapping to mirror typical C signed-int wrap behavior
        i = i.wrapping_sub(1);
    }

    // Print decimal and newline, matching printf("%d\n", j);
    println!("{}", j);
}
