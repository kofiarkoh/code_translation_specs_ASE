use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-like whitespace handling)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C uses long double; Rust has no direct equivalent.
    // Use f64 as the closest stable, safe approximation.
    // This may differ from C on platforms where long double has higher precision.
    let a: f64 = it.next().unwrap_or("0").parse().unwrap_or(0.0);
    let b: f64 = it.next().unwrap_or("0").parse().unwrap_or(0.0);
    let c: f64 = it.next().unwrap_or("0").parse().unwrap_or(0.0);

    let d = c - a - b;

    // Preserve C's floating-point comparison behavior as closely as possible.
    if d * d > 4.0f64 * a * b && d > 0.0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
