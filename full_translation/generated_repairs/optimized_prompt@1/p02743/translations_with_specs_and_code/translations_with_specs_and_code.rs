use std::io::{self, Write};

fn main() {
    // Read input values
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input values as f64
    let values: Vec<&str> = input.trim().split_whitespace().collect();
    let a: f64 = values[0].parse().expect("Invalid input for a");
    let b: f64 = values[1].parse().expect("Invalid input for b");
    let c: f64 = values[2].parse().expect("Invalid input for c");

    // Calculate d
    let d = c - a - b;

    // Check the condition and print the result
    if d * d > 4.0 * a * b && d > 0.0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
