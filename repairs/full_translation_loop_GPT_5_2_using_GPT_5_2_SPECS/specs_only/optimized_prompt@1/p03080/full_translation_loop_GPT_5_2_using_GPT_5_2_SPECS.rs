use std::io::{self, Read};
use std::process;

fn main() {
    // Read entire stdin.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }

    let mut it = input.split_whitespace();

    // If either required input value cannot be read, terminate immediately with non-success status
    // and produce no required output.
    let n_str = match it.next() {
        Some(v) => v,
        None => process::exit(1),
    };
    let token = match it.next() {
        Some(v) => v,
        None => process::exit(1),
    };

    let n: usize = match n_str.parse::<usize>() {
        Ok(v) => v,
        Err(_) => process::exit(1),
    };

    // Preconditions (N non-negative, token has at least N characters, token length <= 127)
    // are assumed by the specification; if violated, behavior is undefined. We do not add checks.

    // Evaluate first N characters. The specification speaks in terms of "characters";
    // we interpret this as bytes of the token as read (ASCII 'R' check), matching typical C behavior.
    let bytes = token.as_bytes();
    let mut score: i32 = 0;
    for i in 0..n {
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
