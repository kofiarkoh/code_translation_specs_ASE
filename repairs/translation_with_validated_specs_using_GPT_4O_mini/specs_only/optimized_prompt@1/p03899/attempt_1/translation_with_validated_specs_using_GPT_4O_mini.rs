fn compute_max_value(n: usize, m: usize, k: usize, values: &[i32]) -> i32 {
    // Preconditions: n, m, k must be non-negative and values must have exactly n elements.
    assert!(n > 0 && m <= n && k > 0);
    assert_eq!(values.len(), n);

    // Dynamic programming array to store maximum values for each iteration
    let mut dp = vec![0; n];

    // Initialize the first iteration with the input values
    for i in 0..n {
        dp[i] = values[i];
    }

    // Process k iterations
    for _ in 1..k {
        let mut new_dp = vec![0; n];

        // Update the new_dp array based on the previous dp values
        for i in 0..n {
            // Find the maximum value from valid previous indices
            let start_index = if i >= m { i - m } else { 0 };
            let end_index = i;

            // Calculate the maximum from the valid range
            let max_prev = dp[start_index..end_index].iter().copied().max().unwrap_or(0);
            new_dp[i] = values[i] + max_prev; // Update new_dp with the current value and max from previous
        }

        // Update dp for the next iteration
        dp = new_dp;
    }

    // Return the maximum value from the last dp array
    *dp.iter().max().unwrap_or(&0)
}
