fn main() {
    // Example inputs
    let n = 5; // Number of elements in the first list
    let initial_list = vec![1, 2, 3, 4, 5]; // List of integers
    let m = 3; // Number of updates
    let updates = vec![(1, 10), (3, 20), (4, 30)]; // List of pairs (index, new value)

    // Ensure preconditions are met
    assert!(n > 0);
    assert!(m >= 0);
    assert!(initial_list.len() == n);
    assert!(updates.len() == m);

    // Calculate the total sum of the initial list
    let total_sum: i32 = initial_list.iter().sum();

    // Prepare a vector to hold the results
    let mut results = Vec::with_capacity(m);

    // Process each update
    for (index, new_value) in updates {
        // Check if the index is valid
        if index < 0 || index >= n as i32 {
            // Behavior is not defined for out-of-bounds index
            // In a real-world scenario, we might want to handle this case
            // For now, we will just continue to the next update
            continue;
        }

        // Calculate the modified sum
        let modified_sum = total_sum - initial_list[index as usize] + new_value;
        results.push(modified_sum);
    }

    // Output the results
    for result in results {
        println!("{}", result);
    }
}
