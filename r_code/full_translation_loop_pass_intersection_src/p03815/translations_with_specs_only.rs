use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    // Unspecified behavior on missing/invalid input: we simply do nothing if parsing fails.
    let n: i64 = match input.split_whitespace().next().and_then(|s| s.parse::<i64>().ok()) {
        Some(v) => v,
        None => return,
    };

    // Rust's / and % on signed integers use truncation toward zero and satisfy:
    // n = 11*q + r, where r has the same sign as n (or is 0), matching the spec.
    let q = n / 11;
    let r = n % 11;

    let out = if r == 0 {
        2 * q
    } else if r > 6 {
        2 * q + 2
    } else {
        2 * q + 1
    };

    println!("{out}");
}
