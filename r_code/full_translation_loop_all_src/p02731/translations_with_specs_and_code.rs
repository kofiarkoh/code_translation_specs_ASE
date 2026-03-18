use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse a single integer.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let l: i32 = input.split_whitespace().next().unwrap().parse().unwrap();

    // Compute cube of one-third of the input as double-precision (f64).
    let y: f64 = (l as f64) / 3.0;
    let x: f64 = y * y * y;

    // Print with exactly twelve digits after the decimal point, newline.
    println!("{:.12}", x);
}
