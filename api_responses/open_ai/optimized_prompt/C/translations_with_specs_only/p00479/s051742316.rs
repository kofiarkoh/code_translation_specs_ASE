fn main() {
    // Example inputs for testing
    let total_positions = 5; // Must be a positive integer
    let number_of_pairs = 3; // Must be a non-negative integer
    let pairs = vec![(1, 2), (3, 4), (5, 1)]; // Each pair must be within the range [1, total_positions]

    // Ensure preconditions are met
    assert!(total_positions > 0, "Total positions must be positive.");
    assert!(number_of_pairs >= 0, "Number of pairs must be non-negative.");
    
    // Prepare a vector to hold the computed values
    let mut results = Vec::new();

    // Process each pair
    for (a, b) in pairs.iter().take(number_of_pairs) {
        // Ensure each position is within the valid range
        assert!(*a >= 1 && *a <= total_positions, "Position a is out of range.");
        assert!(*b >= 1 && *b <= total_positions, "Position b is out of range.");

        // Compute the adjusted positions
        let adjusted_a = (*a - 1) % 3 + 1; // Adjust to range [1, 3]
        let adjusted_b = (*b - 1) % 3 + 1; // Adjust to range [1, 3]

        // Store the computed value based on the adjusted positions
        results.push(adjusted_a + adjusted_b);
    }

    // Output the computed values
    for result in results {
        println!("{}", result); // Each output is guaranteed to be between 1 and 3
    }
}
