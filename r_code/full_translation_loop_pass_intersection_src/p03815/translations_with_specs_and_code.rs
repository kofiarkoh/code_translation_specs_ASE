use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches typical competitive-programming style; no explicit error handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    // Unspecified behavior on missing/invalid input per spec; we simply do nothing if parse fails.
    let x: i64 = match input.split_whitespace().next().and_then(|s| s.parse::<i64>().ok()) {
        Some(v) => v,
        None => return,
    };

    // Rust's / and % on signed integers truncate toward zero, matching the spec's definition of q and r.
    let q = x / 11;
    let r = x % 11;

    let out = if r == 0 {
        2 * q
    } else if r > 6 {
        2 * q + 2
    } else {
        2 * q + 1
    };

    print!("{out}\n");
}
