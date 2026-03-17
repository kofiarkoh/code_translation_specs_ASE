use std::io::{self, Write};

fn main() {
    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have at least two valid integers
    if inputs.len() < 2 {
        eprintln!("Error: At least two valid integers are required.");
        return;
    }

    // Extract the first two integers
    let a = inputs[0];
    let b = inputs[1];

    // Calculate half the product of the first two integers
    // Using i64 to prevent overflow during multiplication
    let product = (a as i64) * (b as i64);
    let half_product = product / 2;

    // Output the result as a formatted integer
    // Ensure the output is non-negative if both inputs are non-negative
    let output = if half_product < 0 {
        0 // If the product is negative, return 0 to satisfy the non-negative invariant
    } else {
        half_product as i32 // Cast back to i32 for output
    };

    // Print the output without decimal places
    println!("{}", output);
}
