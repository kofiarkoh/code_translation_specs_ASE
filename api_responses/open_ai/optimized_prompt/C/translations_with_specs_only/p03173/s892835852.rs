fn min_cost_merge(n: usize, sequence: Vec<i64>) -> i64 {
    // Preconditions: n must be a positive integer and sequence must have exactly n elements.
    assert!(n > 0, "The number of elements must be positive.");
    assert_eq!(sequence.len(), n, "The sequence must contain exactly n elements.");

    // Create a dynamic programming table to store minimum costs
    let mut dp = vec![vec![0; n]; n];

    // Cumulative sum array to help calculate costs efficiently
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + sequence[i];
    }

    // Fill the DP table
    for length in 2..=n {
        for i in 0..=n - length {
            let j = i + length - 1;
            dp[i][j] = i64::MAX; // Initialize to maximum value

            // Try merging at different points
            for k in i..j {
                let cost = dp[i][k] + dp[k + 1][j] + sum[j + 1] - sum[i];
                dp[i][j] = dp[i][j].min(cost);
            }
        }
    }

    // The result is the minimum cost to merge the entire sequence
    dp[0][n - 1]
}

fn main() {
    // Example usage
    let n = 4;
    let sequence = vec![1, 2, 3, 4];
    let result = min_cost_merge(n, sequence);
    println!("Minimum cost of merging: {}", result);
}
