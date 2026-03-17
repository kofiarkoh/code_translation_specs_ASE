fn main() {
    // Example input values
    let n = 5; // Number of elements in the first list
    let initial_list = vec![1, 2, 3, 4, 5]; // The initial list of integers
    let m = 3; // Number of updates
    let updates = vec![(1, 10), (3, 20), (5, 30)]; // List of (index, new_value) pairs

    // Ensure preconditions are met
    assert!(n > 0); // First input integer must be greater than zero
    assert!(m >= 0); // Second input integer must be greater than or equal to zero
    assert!(initial_list.len() == n); // Initial list must match the specified size

    // Calculate the total sum of the initial list
    let total_sum: i32 = initial_list.iter().sum();

    // Prepare a vector to hold the results
    let mut results = Vec::with_capacity(m as usize);

    // Process each update
    for (index, new_value) in updates {
        // Check if the index is valid (0-based index)
        if index < 1 || index > n {
            // Behavior is not defined for out-of-bounds indices
            // In a real application, we might handle this with a panic or error
            continue; // For this implementation, we skip invalid updates
        }

        // Calculate the modified sum
        let modified_sum = total_sum - initial_list[index as usize - 1] + new_value;
        results.push(modified_sum);
    }

    // Output the results
    for result in results {
        println!("{}", result);
    }
}
