fn min_absolute_difference(integers: Vec<u32>, weights: Vec<u32>) -> i32 {
    // Check if the input integers and weights are valid
    if integers.is_empty() || weights.is_empty() {
        return -1; // No valid input
    }

    let num_integers = integers[0] as usize;
    let num_weights = weights[0] as usize;

    // Ensure we have the correct number of integers and weights
    if integers.len() != num_integers + 1 || weights.len() != num_weights + 1 {
        return -1; // Invalid input size
    }

    let base_value = 0; // Base value for computed values
    let mut computed_values = Vec::new();

    // Generate all combinations of computed values from weights
    for i in 0..(1 << num_weights) {
        let mut computed_value = base_value;
        for j in 0..num_weights {
            if (i & (1 << j)) != 0 {
                computed_value += weights[j + 1]; // Add weight
            } else {
                computed_value -= weights[j + 1]; // Subtract weight
            }
        }
        // Only store non-negative computed values
        if computed_value >= 0 {
            computed_values.push(computed_value);
        }
    }

    let mut min_difference = i32::MAX; // Initialize to maximum possible value
    let mut found_valid_difference = false; // Track if any valid difference is found

    // Compare input integers with computed values
    for &integer in &integers[1..] {
        for &computed in &computed_values {
            let difference = (integer as i32 - computed as i32).abs();
            if difference < min_difference {
                min_difference = difference;
                found_valid_difference = true; // Found at least one valid difference
            }
        }
    }

    // Return the minimum difference or -1 if none found
    if found_valid_difference {
        min_difference
    } else {
        -1
    }
}

fn main() {
    // Example usage
    let integers = vec![5, 1, 2, 3, 4, 5]; // First element is the count
    let weights = vec![3, 1, 2, 3]; // First element is the count
    let result = min_absolute_difference(integers, weights);
    println!("{}", result); // Output the result
}
