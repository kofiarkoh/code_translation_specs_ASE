use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i64 (matches C's long long on typical platforms).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let x: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    // Preserve C semantics:
    // - `%` and `/` use truncation toward zero in both C (C99+) and Rust.
    // - Remainder has the same sign as the dividend in both languages.
    let r = x % 11;
    let q = x / 11;

    let out = if r == 0 {
        2 * q
    } else if r > 6 {
        2 * q + 2
    } else {
        2 * q + 1
    };

    print!("{out}\n");
}
