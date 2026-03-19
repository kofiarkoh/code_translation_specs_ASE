use std::io::{self, Read};
use std::process;

fn main() {
    // Read entire stdin, then parse as whitespace-delimited tokens (like scanf).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }
    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|t| t.parse::<i32>().ok()) {
        Some(v) => v,
        None => process::exit(1), // required input cannot be read
    };
    let s = match it.next() {
        Some(v) => v,
        None => process::exit(1), // required input cannot be read
    };

    // Preconditions (non-negative N, token has at least N chars, length <= 127) are assumed.
    // If violated, behavior is unspecified per spec; we do not add extra checks.
    let n_usize = n as usize;

    let bytes = s.as_bytes();
    let mut score: i32 = 0;
    for i in 0..n_usize {
        if bytes[i] == b'R' {
            score += 1;
        } else {
            score -= 1;
        }
    }

    if score > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
