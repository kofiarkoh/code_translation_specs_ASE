use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input into three f64 values
    let mut values = input.split_whitespace()
        .map(|s| s.parse::<f64>().expect("Failed to parse number"))
        .collect::<Vec<f64>>();

    // Unwrap the values into a, b, c
    let (a, b, c) = (values[0], values[1], values[2]);
    
    // Calculate d
    let d = c - a - b; // d = c - a - b

    // Check the condition and print the result
    if d * d > 4.0 * a * b && d > 0.0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
