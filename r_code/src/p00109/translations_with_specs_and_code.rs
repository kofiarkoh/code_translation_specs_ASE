// expr_eval.rs
//
// Translation of the provided C evaluator to Rust following the SPEC.
// - Recursive-descent parser for expressions with +, -, *, / and parentheses.
// - Reads first token n, then n whitespace-separated expression tokens from stdin.
// - For each token prints one line with the evaluated integer result.
// Notes:
// - This implementation closely follows the C semantics described in the SPEC.
// - Add/sub/mul use wrapping operations to avoid Rust overflow panics and to
//   be permissive in a manner similar to C's undefined signed overflow (the SPEC
//   treats signed overflow as undefined). Division uses standard i32 division;
//   dividing by zero will panic (matching the SPEC's "undefined" behavior).
// - No validation of malformed expressions is performed (as in the C code).

use std::io::{self, Read};

fn peek_byte(bytes: &[u8], pos: usize) -> u8 {
    if pos < bytes.len() {
        bytes[pos]
    } else {
        b'\0'
    }
}

fn number(bytes: &[u8], pos: &mut usize) -> i32 {
    let mut val: i32 = 0;
    while {
        let c = peek_byte(bytes, *pos);
        c >= b'0' && c <= b'9'
    } {
        let digit = (peek_byte(bytes, *pos) - b'0') as i32;
        // use wrapping to avoid Rust debug panic on overflow; keeps behavior permissive
        val = val.wrapping_mul(10).wrapping_add(digit);
        *pos += 1;
    }
    val
}

fn factor(bytes: &[u8], pos: &mut usize) -> i32 {
    if peek_byte(bytes, *pos) == b'(' {
        *pos += 1; // consume '('
        let val = expr(bytes, pos);
        // Corresponds to the C code that unconditionally does (*s)++
        *pos = pos.saturating_add(1);
        val
    } else {
        number(bytes, pos)
    }
}

fn term(bytes: &[u8], pos: &mut usize) -> i32 {
    let mut val = factor(bytes, pos);
    loop {
        match peek_byte(bytes, *pos) {
            b'*' => {
                *pos += 1;
                let rhs = factor(bytes, pos);
                val = val.wrapping_mul(rhs);
            }
            b'/' => {
                *pos += 1;
                let rhs = factor(bytes, pos);
                // Use standard division; division by zero will panic (matches SPEC's undefined behavior)
                val = val / rhs;
            }
            _ => break,
        }
    }
    val
}

fn expr(bytes: &[u8], pos: &mut usize) -> i32 {
    let mut val = term(bytes, pos);
    loop {
        match peek_byte(bytes, *pos) {
            b'+' => {
                *pos += 1;
                let rhs = term(bytes, pos);
                val = val.wrapping_add(rhs);
            }
            b'-' => {
                *pos += 1;
                let rhs = term(bytes, pos);
                val = val.wrapping_sub(rhs);
            }
            _ => break,
        }
    }
    val
}

fn main() -> io::Result<()> {
    // Read all stdin, split on whitespace (behaves like scanf("%s") for tokens).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut toks = input.split_whitespace();

    // Read n (first token).
    let n_token = match toks.next() {
        Some(t) => t,
        None => return Ok(()), // no input; nothing to do (spec does not mandate error handling)
    };

    // Parse n as i32. If parsing fails, behave permissively and treat as 0.
    let n: i32 = n_token.parse().unwrap_or(0);

    // For each of the next n tokens, evaluate and print result.
    for _ in 0..n {
        let expr_token = toks.next().unwrap_or("");
        let bytes = expr_token.as_bytes();
        let mut pos: usize = 0;
        let val = expr(bytes, &mut pos);
        println!("{}", val);
    }

    Ok(())
}
