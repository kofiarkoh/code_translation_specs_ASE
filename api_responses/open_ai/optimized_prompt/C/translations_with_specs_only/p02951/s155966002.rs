use std::io::{self, Write};

fn main() {
    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok()) // Parse integers, ignoring invalid inputs
        .collect();

    // Ensure we have exactly three valid integers
    if numbers.len() != 3 {
        eprintln!("Error: Expected three valid integers.");
        return; // Undefined behavior if not three integers
    }

    let first = numbers[0];
    let second = numbers[1];
    let mut third = numbers[2];

    // Subtract the second integer from the first
    let result = first - second;

    // Decrement the third integer based on the result
    if result > 0 {
        // Ensure third does not go below zero
        third = third.saturating_sub(1);
    }

    // Output the final value of the modified third integer
    println!("{}", third);
}
