use std::io::{self, BufRead};

fn main() {
    // Read a line of input from standard input
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    // Split the line into parts and parse them as f64 (closest to long double in Rust)
    let mut values = line.split_whitespace()
        .map(|s| s.parse::<f64>().unwrap()) // Assuming valid input as per specification
        .collect::<Vec<f64>>();

    // Unwrap the values into a, b, and c
    let a = values.remove(0);
    let b = values.remove(0);
    let c = values.remove(0);

    // Calculate d
    let d = c - a - b;

    // Check the condition d^2 > 4ab and d > 0
    if d > 0.0 && d * d > 4.0 * a * b {
        println!("Yes");
    } else {
        println!("No");
    }
}
