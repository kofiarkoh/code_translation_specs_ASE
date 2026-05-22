use std::io::{self, Read};
use std::process;

fn main() {
    // Read entire stdin (whitespace-delimited parsing like scanf).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }
    let mut it = input.split_whitespace();

    let n: usize = match it.next().and_then(|t| t.parse::<usize>().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    let s = match it.next() {
        Some(v) => v,
        None => process::exit(1),
    };

    // C code uses a fixed 128-byte buffer and reads at most 127 chars.
    // The spec notes a fixed upper bound; we mirror the C behavior by truncating.
    let s_trunc = if s.len() > 127 { &s[..127] } else { s };

    // Behavior is undefined if n exceeds available sequence length; we do not guard.
    let bytes = s_trunc.as_bytes();

    let mut total: i32 = 0;
    for i in 0..n {
        if bytes[i] == b'R' {
            total += 1;
        } else {
            total -= 1;
        }
    }

    if total > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
