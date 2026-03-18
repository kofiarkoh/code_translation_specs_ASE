use std::collections::HashSet;

fn minimum_absolute_difference(input: Vec<usize>) -> i32 {
    // Check if the input is valid based on the specification
    if input.len() < 3 {
        return -1; // Not enough integers to process
    }

    let n = input[0]; // Number of integers
    let m = input[1]; // Number of weights

    // Validate the number of integers and weights
    if n == 0 || m == 0 || input.len() != n + 2 {
        return -1; // Invalid input format
    }

    let values: Vec<usize> = input[2..2 + n].to_vec(); // Extract values
    let weights: Vec<usize> = input[2 + n..2 + n + m].to_vec(); // Extract weights

    // Set to maintain computed values
    let mut computed_values = HashSet::new();

    // Compute values based on weights
    for &weight in &weights {
        computed_values.insert(weight); // Store each weight as a computed value
    }

    // Initialize minimum difference to a large value
    let mut min_diff = usize::MAX;

    // Check each value against the computed values
    for &value in &values {
        for &computed in &computed_values {
            let diff = value.abs_diff(computed); // Calculate absolute difference
            if diff < min_diff {
                min_diff = diff; // Update minimum difference
            }
        }
    }

    // Return the minimum difference or -1 if no valid difference found
    if min_diff == usize::MAX {
        -1
    } else {
        min_diff as i32 // Convert to i32 for output
    }
}

// Example usage
fn main() {
    let input = vec![5, 3, 10, 20, 30, 40, 5, 15, 25]; // Example input
    let result = minimum_absolute_difference(input);
    println!("{}", result); // Output the result
}
