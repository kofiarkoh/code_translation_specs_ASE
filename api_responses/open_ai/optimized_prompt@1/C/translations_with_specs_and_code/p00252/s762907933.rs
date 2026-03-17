use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input values
    let values: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Input must be integers"))
        .collect();

    // Ensure we have exactly three values
    if values.len() != 3 {
        eprintln!("Error: Expected exactly three input values.");
        return;
    }

    let (a, b, c) = (values[0], values[1], values[2]);

    // Check preconditions: a, b, c must be either 0 or 1
    if !is_valid_binary(a) || !is_valid_binary(b) || !is_valid_binary(c) {
        eprintln!("Error: Input values must be either 0 or 1.");
        return;
    }

    // Determine the output based on the input values
    let output = match (a, b, c) {
        (1, 0, 0) => "Close",
        (0, 1, 0) => "Close",
        (1, 1, 0) => "Open",
        (0, 0, 1) => "Open",
        (0, 0, 0) => "Close",
        _ => unreachable!(), // All cases are covered by the match
    };

    // Print the output
    println!("{}", output);
}

// Helper function to check if a value is binary (0 or 1)
fn is_valid_binary(value: i32) -> bool {
    value == 0 || value == 1
}
