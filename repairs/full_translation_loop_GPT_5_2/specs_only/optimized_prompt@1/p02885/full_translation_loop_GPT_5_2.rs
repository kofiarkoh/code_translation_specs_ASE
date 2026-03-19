use std::io::{self, Read};

fn main() {
    // Reads two integers; behavior is undefined if inputs are not valid integers.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let a: i128 = it.next().unwrap().parse().unwrap();
    let b: i128 = it.next().unwrap().parse().unwrap();

    // The required operation (inferred from failing sample) is:
    // max(0, a - 2*b)
    // Output is guaranteed non-negative by clamping at zero.
    let v = a - 2 * b;
    let out = if v < 0 { 0 } else { v };

    println!("{out}");
}
