use std::io::{self, Read};

fn main() {
    // Read all stdin and split by whitespace into tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut toks = input.split_whitespace();

    // Parse first token as n (C signed int -> i32 here).
    // The specification assumes at least one token is present; unwrap() matches the unspecified behavior otherwise.
    let n_token = toks.next().unwrap();
    let n: i32 = n_token.parse().unwrap();

    // If n <= 0, do nothing (no output lines), per specification.
    if n <= 0 {
        return;
    }

    // Process n cases. The specification assumes there are at least n further integer tokens;
    // if not, behavior is unspecified (we allow unwrap() to panic).
    for _case_idx in 0..(n as usize) {
        let m_token = toks.next().unwrap();
        // Parse m as a signed C int (i32).
        let mut m: i32 = m_token.parse().unwrap();

        // Extract exactly 8 least-significant decimal digits by repeated % and /,
        // matching typical C semantics (truncation toward zero for /, remainder sign follows dividend).
        // Rust's / and % on signed integers use truncation toward zero and remainder with sign of dividend,
        // which aligns with common C behaviour (C99+).
        let mut digits = [0i32; 8];
        for i in 0..8 {
            let d = m % 10; // may be negative if m is negative; behaviour left as in spec (unspecified for negatives)
            digits[i] = d;
            m = m / 10;
        }

        // Sort ascending.
        let mut v: Vec<i32> = digits.to_vec();
        v.sort();

        // Build Min8 by concatenating digits in ascending order: (((d0)*10 + d1)*10 + d2) ...
        let mut min8: i32 = 0;
        for &d in &v {
            min8 = min8 * 10 + d;
        }

        // Build Max8 by concatenating digits in descending order.
        let mut max8: i32 = 0;
        for &d in v.iter().rev() {
            max8 = max8 * 10 + d;
        }

        // Compute difference and print one line per case.
        let diff = max8 - min8;
        println!("{}", diff);
    }
}
