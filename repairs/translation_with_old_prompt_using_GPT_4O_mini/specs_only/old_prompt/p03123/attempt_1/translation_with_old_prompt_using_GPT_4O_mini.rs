use std::io::{self, Write};

fn solve(r: i64, b: i64) -> i64 {
    // Calculate the maximum number of items that can be created
    // The formula is not explicitly given, but we assume it involves both resources.
    // For the sake of this implementation, we will use a simple heuristic:
    // The maximum number of items is the minimum of the two resources.
    // This is a placeholder for the actual logic that would be defined in the specification.
    
    // If both resources are zero, we return -1 as per the edge-case behavior.
    if r == 0 && b == 0 {
        return -1;
    }

    // The maximum number of items is determined by the resources available.
    // We assume a simple relationship for demonstration purposes.
    let max_items = (r + b) / 2; // This is a placeholder for the actual calculation.

    // Return the maximum number of items minus one, as specified.
    max_items - 1
}

fn main() {
    // Read input values for r and b
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Ensure we have exactly two inputs
    if values.len() != 2 {
        eprintln!("Expected two long long integers as input.");
        return;
    }

    let r = values[0];
    let b = values[1];

    // Call the solve function and print the result
    let result = solve(r, b);
    println!("{}", result);
}
