use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line for number of pairs and threshold
    let first_line = lines.next().expect("Failed to read the first line");
    let first_line = first_line.expect("Failed to unwrap the first line");
    let mut inputs = first_line.split_whitespace();
    
    // Parse number of pairs and threshold
    let num_pairs: usize = inputs.next().expect("Failed to read number of pairs").parse().expect("Failed to parse number of pairs");
    let threshold: i32 = inputs.next().expect("Failed to read threshold").parse().expect("Failed to parse threshold");

    // Ensure we do not exceed the maximum allowed pairs
    assert!(num_pairs > 0 && num_pairs <= 1000);
    assert!(threshold > 0);

    let mut adjusted_values = Vec::new();

    // Process each pair
    for _ in 0..num_pairs {
        let line = lines.next().expect("Failed to read a pair line");
        let line = line.expect("Failed to unwrap a pair line");
        let mut pair_inputs = line.split_whitespace();
        
        // Read the pair of integers
        let first: i32 = pair_inputs.next().expect("Failed to read first integer").parse().expect("Failed to parse first integer");
        let second: i32 = pair_inputs.next().expect("Failed to read second integer").parse().expect("Failed to parse second integer");

        // Check for non-negative values
        if first < 0 || second < 0 {
            continue; // Skip invalid pairs
        }

        // Adjust the first value based on the threshold
        let adjusted_value = if first > threshold {
            first - threshold
        } else {
            0
        };

        // Store the adjusted value
        adjusted_values.push(adjusted_value);
    }

    // Compute the sum of adjusted values
    let sum: i32 = adjusted_values.iter().sum();

    // If all pairs are invalidated, the output will be zero
    let output = if adjusted_values.is_empty() { 0 } else { sum };

    // Print the result
    println!("{}", output);
}
