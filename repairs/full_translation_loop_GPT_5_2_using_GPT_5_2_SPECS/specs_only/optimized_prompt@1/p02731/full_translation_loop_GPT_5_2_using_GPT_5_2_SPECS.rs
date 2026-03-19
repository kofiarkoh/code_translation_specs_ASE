use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Reads one integer; behavior is unspecified if it cannot be read.
    let l: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    // Compute (l/3)^3 in floating-point arithmetic.
    let x = (l as f64) / 3.0;
    let y = x * x * x;

    // Match expected formatting: fixed-point with 12 digits after the decimal point.
    println!("{:.12}", y);
}
