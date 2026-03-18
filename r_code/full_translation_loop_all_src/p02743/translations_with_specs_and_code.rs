use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches typical competitive-programming style I/O).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: f64 = it.next().unwrap().parse().unwrap();
    let b: f64 = it.next().unwrap().parse().unwrap();
    let c: f64 = it.next().unwrap().parse().unwrap();

    // C uses long double; Rust stable has no long double type.
    // We use f64 as the closest standard floating type.
    let d = c - a - b;

    if d > 0.0 && d * d > 4.0 * a * b {
        print!("Yes\n");
    } else {
        print!("No\n");
    }
}
