fn min_cost_to_combine_weights(n: usize, weights: Vec<usize>) -> usize {
    // Preconditions: n must be between 1 and 4000, and weights must have exactly n non-negative integers.
    assert!(n >= 1 && n <= 4000);
    assert_eq!(weights.len(), n);

    // Initialize the dynamic programming table
    let mut dp = vec![vec![usize::MAX; n]; n];

    // Base case: cost of combining a single weight is 0
    for i in 0..n {
        dp[i][i] = 0;
    }

    // Precompute the prefix sums for quick range sum calculation
    let mut prefix_sum = vec![0; n + 1];
    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + weights[i];
    }

    // Fill the dynamic programming table
    for length in 2..=n { // length of the range
        for i in 0..=n - length { // starting index
            let j = i + length - 1; // ending index
            // Calculate the cost of combining weights from i to j
            for k in i..j { // possible split points
                let cost = prefix_sum[j + 1] - prefix_sum[i]; // sum of weights from i to j
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k + 1][j] + cost);
            }
        }
    }

    // The result is the minimum cost to combine all weights
    dp[0][n - 1]
}

fn main() {
    // Example usage
    let n = 4;
    let weights = vec![1, 2, 3, 4];
    let result = min_cost_to_combine_weights(n, weights);
    println!("Minimum cost to combine weights: {}", result);
}
