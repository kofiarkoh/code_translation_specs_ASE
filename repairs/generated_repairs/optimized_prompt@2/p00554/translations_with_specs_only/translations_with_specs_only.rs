fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line containing the number of pairs and the threshold
    let first_line = lines.next().unwrap_or_else(|_| panic!("Failed to read the first line")).unwrap();
    let mut inputs = first_line.split_whitespace();
    
    // Parse the number of pairs and the threshold
    let num_pairs: usize = inputs.next().unwrap().parse().unwrap();
    let threshold: i32 = inputs.next().unwrap().parse().unwrap();

    // Ensure we do not exceed the maximum allowed pairs
    assert!(num_pairs > 0 && num_pairs <= 1000);
    assert!(threshold > 0);

    let mut adjusted_values = Vec::new();
    let mut invalid_count = 0;

    // Process each pair of integers
    for _ in 0..num_pairs {
        let line = lines.next().unwrap_or_else(|| panic!("Failed to read a line for pair")).unwrap();
        let mut pair_inputs = line.split_whitespace();
        
        // Read the pair of integers
        let first: i32 = pair_inputs.next().unwrap().parse().unwrap();
        let second: i32 = pair_inputs.next().unwrap().parse().unwrap();

        // Check for non-negative integers
        if first < 0 || second < 0 {
            invalid_count += 1;
            continue; // Skip invalid pairs
        }

        // Adjust the first integer based on the threshold
        let adjusted_value = if first > threshold {
            first - threshold
        } else {
            0 // If first is not greater than threshold, it is adjusted to 0
        };

        // Store the adjusted value
        adjusted_values.push(adjusted_value);
    }

    // Compute the sum of adjusted values
    let sum: i32 = adjusted_values.iter().sum();

    // If all pairs are invalidated, the output will be zero
    let output = if invalid_count == num_pairs { 0 } else { sum };

    // Output the result
    println!("{}", output);
}
