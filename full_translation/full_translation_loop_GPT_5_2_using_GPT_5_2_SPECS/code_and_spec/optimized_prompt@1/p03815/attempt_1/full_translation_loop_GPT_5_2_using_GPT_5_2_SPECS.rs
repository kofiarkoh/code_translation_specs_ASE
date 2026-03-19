use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse one signed 64-bit integer.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let x: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    // Use Rust's / and % on i64, which match C's truncation-toward-zero division
    // and corresponding remainder sign behavior.
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
