fn compute_weighted_sums(inputs: Vec<(usize, usize, Vec<usize>, Vec<usize>)>) -> Vec<i32> {
    // Initialize a vector to hold the results for each input pair
    let mut results = Vec::new();

    // Process each input pair
    for (count1, _count2, set1, set2) in inputs { // Prefixing _count2 to silence the unused variable warning
        // Check for termination condition
        if count1 == 0 {
            break; // Terminate processing if the first count is zero
        }

        // Initialize count arrays for integers 1 to 20
        let mut counts1 = [0; 21]; // Index 0 will be unused
        let mut counts2 = [0; 21]; // Index 0 will be unused

        // Populate counts for the first set
        for &num in &set1 {
            counts1[num] += 1; // Increment count for the respective integer
        }

        // Populate counts for the second set
        for &num in &set2 {
            counts2[num] += 1; // Increment count for the respective integer
        }

        // Calculate the weighted sum
        let mut weighted_sum = 0;
        for i in 1..=20 {
            let max_count = counts1[i].max(counts2[i]); // Get the maximum count for the integer
            weighted_sum += max_count as i32 * i as i32; // Multiply by the integer value and accumulate
        }

        // Store the result for the current input pair
        results.push(weighted_sum);
    }

    results // Return the vector of results
}

fn main() {
    // Example usage of the compute_weighted_sums function
    let inputs = vec![
        (3, 3, vec![1, 2, 3], vec![2, 3, 4]),
        (0, 0, vec![], vec![]), // This will terminate the processing
    ];
    
    let results = compute_weighted_sums(inputs);
    for result in results {
        println!("{}", result); // Output the results
    }
}
