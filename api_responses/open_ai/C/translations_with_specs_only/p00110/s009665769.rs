// cryptarithm_solver.rs
//
// Reads whitespace-separated tokens of the form "<left>+<right>=<result>"
// and for each token finds the smallest decimal digit 0..9 that, when
// substituted for every 'X' in the three operands, makes left + right == result.
// If none found, prints "NA".
//
// Notes on semantics faithful to the specification:
// - Empty operands are interpreted as numeric 0.
// - Leading zeros are allowed.
// - The same digit is substituted for every 'X' in all operands.
// - Integer arithmetic uses wrapping arithmetic on u128 to emulate possible
//   overflow/wrap-around behavior (the specification allows undefined overflow).
// - Tokens that don't contain '+' and '=' in the required order, or that
//   contain invalid characters, result in printing "NA" for that token.
//   The specification permits undefined behavior for malformed input, but
//   this program chooses the narrow, safe behavior of emitting "NA".

use std::io::{self, Read};

fn compute_with_digit(operand: &str, digit_char: char) -> Option<u128> {
    // Interpret operand as a base-10 integer after substituting 'X' -> digit_char.
    // Return None if operand contains invalid characters.
    let mut acc: u128 = 0;
    let digit_val_for_x = (digit_char as u8).wrapping_sub(b'0') as u128;

    for c in operand.chars() {
        let d = if c == 'X' {
            digit_val_for_x
        } else if ('0'..='9').contains(&c) {
            (c as u8 - b'0') as u128
        } else {
            // Invalid character: signal failure
            return None;
        };
        // Use wrapping arithmetic to emulate possible overflow semantics.
        acc = acc.wrapping_mul(10).wrapping_add(d);
    }
    Some(acc)
}

fn solve_token(token: &str) -> String {
    // Find '+' and '=', ensure order left + right = result
    let plus_pos = token.find('+');
    let eq_pos = token.find('=');

    if plus_pos.is_none() || eq_pos.is_none() {
        return "NA".to_string();
    }
    let plus = plus_pos.unwrap();
    let eq = eq_pos.unwrap();
    if plus >= eq {
        return "NA".to_string();
    }

    let left = &token[..plus];
    let right = &token[plus + 1..eq];
    let result = &token[eq + 1..];

    for d in 0u8..=9 {
        let digit_char = (b'0' + d) as char;
        // Compute substituted numeric values
        let lv = compute_with_digit(left, digit_char);
        let rv = compute_with_digit(right, digit_char);
        let rv2 = compute_with_digit(result, digit_char);
        if lv.is_none() || rv.is_none() || rv2.is_none() {
            // invalid characters; per spec behavior is undefined.
            // Here we choose to treat as no candidate match for this digit.
            continue;
        }
        let l = lv.unwrap();
        let r = rv.unwrap();
        let res = rv2.unwrap();
        // Use wrapping add for left + right
        if l.wrapping_add(r) == res {
            return digit_char.to_string();
        }
    }

    "NA".to_string()
}

fn main() {
    // Read entire stdin, split on whitespace into tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we cannot read stdin, exit quietly (no output).
        return;
    }

    // For each whitespace-separated token, process and print one line.
    let mut first = true;
    for token in input.split_whitespace() {
        // The specification expects one output line per token.
        if !first {
            // println! automatically adds newlines; nothing special needed.
        }
        first = false;
        let out = solve_token(token);
        println!("{}", out);
    }
}
