use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-style tokenization).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|t| t.parse::<i32>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    let s_token = match it.next() {
        Some(v) => v,
        None => std::process::exit(1),
    };

    // C: char s[128]; scanf("%127s", s) => at most 127 bytes + '\0'.
    // We'll emulate by truncating to 127 bytes (ASCII expected for 'R' checks).
    let s_bytes = s_token.as_bytes();
    let s_trunc = if s_bytes.len() > 127 { &s_bytes[..127] } else { s_bytes };

    let mut kitaeri: i32 = 0;

    // C loop: for (i = 0; i < N; i++) { if (s[i] == 'R') ... }
    // If N exceeds the string length, C reads beyond the terminator (UB).
    // We cannot reproduce UB in safe Rust; we clamp to available bytes.
    let limit = if n <= 0 {
        0usize
    } else {
        let nu = n as usize;
        if nu > s_trunc.len() { s_trunc.len() } else { nu }
    };

    for i in 0..limit {
        if s_trunc[i] == b'R' {
            kitaeri += 1;
        } else {
            kitaeri -= 1;
        }
    }

    if kitaeri > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
