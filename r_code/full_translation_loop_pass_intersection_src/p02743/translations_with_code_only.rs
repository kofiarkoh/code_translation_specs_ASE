use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-style whitespace separated parsing)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C uses long double; in Rust we approximate with f64 (platform-independent).
    // This preserves the intended floating-point behavior as closely as possible
    // without relying on non-portable extended precision.
    let a: f64 = it.next().unwrap().parse().unwrap();
    let b: f64 = it.next().unwrap().parse().unwrap();
    let c: f64 = it.next().unwrap().parse().unwrap();

    let d = c - a - b;

    if d * d > 4.0f64 * a * b && d > 0.0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
