fn max_value(num_items: usize, resource_limit1: usize, resource_limit2: usize, items: &[(usize, usize, usize)]) -> usize {
    // Create a dynamic programming table initialized to 0
    let mut dp = vec![vec![0; resource_limit2 + 1]; resource_limit1 + 1];

    // Iterate through each item
    for &(cost1, cost2, value) in items.iter() {
        // Update the DP table in reverse to avoid overwriting results from the current iteration
        for r1 in (cost1..=resource_limit1).rev() {
            for r2 in (cost2..=resource_limit2).rev() {
                // Update the maximum value obtainable
                dp[r1][r2] = dp[r1][r2].max(dp[r1 - cost1][r2 - cost2] + value);
            }
        }
    }

    // The maximum value obtainable is found at the bottom-right corner of the DP table
    dp[resource_limit1][resource_limit2]
}

fn main() {
    // Example usage
    let items = vec![(1, 2, 3), (2, 3, 4), (3, 4, 5)];
    let max_val = max_value(items.len(), 5, 5, &items);
    println!("Maximum obtainable value: {}", max_val);
}
