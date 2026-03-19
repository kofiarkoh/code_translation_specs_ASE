use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Specification: reads one integer value; behavior unspecified if it cannot be read.
    let l: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    // Compute (l/3)^3 in floating-point real arithmetic (subject to rounding).
    let x = (l as f64) / 3.0;
    let y = x * x * x;

    // Output one real-number result followed by a newline.
    println!("{}", y);
}
