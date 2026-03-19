use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let x: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    // Rust's / and % on signed integers use truncation toward zero and produce a remainder r
    // such that x = 11*q + r, matching the specification.
    let q = x / 11;
    let r = x % 11;

    let out = if r == 0 {
        2 * q
    } else if r > 6 {
        2 * q + 2
    } else {
        2 * q + 1
    };

    println!("{out}");
}
