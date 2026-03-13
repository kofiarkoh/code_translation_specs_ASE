// expr_eval.rs
//
// A faithful translation of the given specification into Rust.
// Evaluates N whitespace-separated expression tokens from stdin and prints
// one line per expression with the computed 32-bit C int result.
//
// Notes on semantic choices to match the Specification:
// - Arithmetic is performed with 32-bit C "int" semantics as closely as possible.
//   We use i32 for values and use wrapping for add/sub/mul to emulate typical
//   two's-complement behavior when overflow would occur.
// - Division semantics: we perform division by promoting to i64, dividing
//   (which implements truncation toward zero), then casting back to i32.
//   This reproduces the natural truncation behavior and causes the MIN/-1
//   case to wrap modulo 2^32 (matching typical two's-complement behavior).
// - No special validation or error handling is performed for malformed expressions,
//   division by zero, or overly long tokens, per the Specification.
// - Unary +/− are not supported; numbers are sequences of digits only.
// - Operator precedence and left associativity are implemented per grammar.
//
// The program reads all whitespace-separated tokens from stdin. The first token
// is parsed as the integer n (like scanf("%d",&n)), then the next n tokens are
// each treated as an expression string (like scanf("%s", s)).

use std::io::{self, Read};

type Int = i32;

fn main() {
    // Read entire stdin and split on whitespace to emulate scanf("%d" ...) then scanf("%s", s)...
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, nothing to do (Specification does not require diagnostics).
        return;
    }

    let mut tokens = input.split_whitespace();
    // Parse first token as n, similar to scanf("%d", &n).
    // If parsing fails, treat as 0 (nonspecified behavior for malformed input).
    let n: i32 = match tokens.next() {
        Some(t) => t.parse::<i32>().unwrap_or(0),
        None => return,
    };

    // For each of the next n tokens, evaluate expression and print result.
    // Specification expects n to be non-negative; if negative, behavior is unspecified.
    if n <= 0 {
        return;
    }
    let mut remaining = n;
    while remaining > 0 {
        let expr_token = tokens.next().unwrap_or("");
        let val = eval_expression(expr_token);
        // Print as decimal with newline, equivalent to printf("%d\n", val).
        println!("{}", val);
        remaining -= 1;
    }
}

// Top-level evaluator for an expression string.
// Returns an Int (i32). For malformed input behavior is unspecified (we try to do reasonable thing).
fn eval_expression(s: &str) -> Int {
    let bytes = s.as_bytes();
    let mut pos: usize = 0;
    parse_expr(bytes, &mut pos)
}

// parse_expr := parse_term { ('+'|'-') parse_term }
fn parse_expr(s: &[u8], pos: &mut usize) -> Int {
    let mut acc = parse_term(s, pos);
    loop {
        skip_none(s, pos);
        if let Some(b) = peek_char(s, *pos) {
            match b {
                b'+' => {
                    *pos += 1;
                    let rhs = parse_term(s, pos);
                    acc = acc.wrapping_add(rhs);
                }
                b'-' => {
                    *pos += 1;
                    let rhs = parse_term(s, pos);
                    acc = acc.wrapping_sub(rhs);
                }
                _ => break,
            }
        } else {
            break;
        }
    }
    acc
}

// parse_term := parse_factor { ('*'|'/') parse_factor }
fn parse_term(s: &[u8], pos: &mut usize) -> Int {
    let mut acc = parse_factor(s, pos);
    loop {
        skip_none(s, pos);
        if let Some(b) = peek_char(s, *pos) {
            match b {
                b'*' => {
                    *pos += 1;
                    let rhs = parse_factor(s, pos);
                    acc = acc.wrapping_mul(rhs);
                }
                b'/' => {
                    *pos += 1;
                    let rhs = parse_factor(s, pos);
                    // Division: emulate C-like truncation toward zero and typical two's-complement wrap
                    // for the overflow case by computing in i64 and casting back to i32.
                    acc = c_like_div(acc, rhs);
                }
                _ => break,
            }
        } else {
            break;
        }
    }
    acc
}

// parse_factor := number | '(' parse_expr ')'
fn parse_factor(s: &[u8], pos: &mut usize) -> Int {
    skip_none(s, pos);
    if let Some(b) = peek_char(s, *pos) {
        if b == b'(' {
            *pos += 1; // consume '('
            let val = parse_expr(s, pos);
            // If there's a ')', consume it. If not, leave as-is (behavior unspecified).
            skip_none(s, pos);
            if let Some(b2) = peek_char(s, *pos) {
                if b2 == b')' {
                    *pos += 1;
                }
            }
            return val;
        } else {
            return parse_number(s, pos);
        }
    }
    // No factor: unspecified per spec; return 0 as a conservative choice.
    0
}

// parse_number: consumes contiguous digits and returns their integer value.
// Uses wrapping arithmetic to emulate C 32-bit int accumulation overflow behavior.
fn parse_number(s: &[u8], pos: &mut usize) -> Int {
    let mut val: Int = 0;
    let mut any = false;
    while let Some(b) = peek_char(s, *pos) {
        if b'0' <= b && b <= b'9' {
            any = true;
            *pos += 1;
            let digit = (b - b'0') as Int;
            val = val.wrapping_mul(10).wrapping_add(digit);
        } else {
            break;
        }
    }
    // If no digits found (malformed), behavior is unspecified; return 0 conservatively.
    if !any {
        0
    } else {
        val
    }
}

// Helper: peek next byte at position, or None if out of bounds.
fn peek_char(s: &[u8], pos: usize) -> Option<u8> {
    if pos < s.len() {
        Some(s[pos])
    } else {
        None
    }
}

// Helper: skip over nothing (placeholder for potential whitespace inside token).
// In this spec, expressions have no internal whitespace, so this is a no-op.
// Kept for structural similarity to recursive-descent parser skippers.
fn skip_none(_s: &[u8], _pos: &mut usize) {
    // Intentionally empty: tokens contain no whitespace per specification.
}

// C-like division emulation:
// - Uses i64 division to perform truncation toward zero reliably.
// - Casts back to i32 so that overflow (e.g. MIN / -1) wraps modulo 2^32, matching common two's-complement results.
// - Division by zero will panic at runtime (unspecified per spec).
fn c_like_div(lhs: Int, rhs: Int) -> Int {
    // Promote to i64 for division; this preserves truncation toward zero semantics.
    let a = lhs as i64;
    let b = rhs as i64;
    // Let i64 division happen; if b == 0 this will panic (spec allows undefined behavior).
    let q = a / b;
    // Cast back to i32, performing two's-complement wrap if out of range.
    q as i32
}
