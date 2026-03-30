fn min_cost_to_combine_weights(n: usize, weights: Vec<usize>) -> usize {
    // Preconditions: n must be between 1 and 4000, and weights must have exactly n non-negative integers.
    assert!(n >= 1 && n <= 4000);
    assert_eq!(weights.len(), n);

    // Initialize the dynamic programming table
    let mut dp = vec![vec![0; n]; n];

    // Fill the dynamic programming table
    for length in 2..=n { // length of the range
        for i in 0..=n - length { // starting index
            let j = i + length - 1; // ending index
            // Initialize the minimum cost for this range
            dp[i][j] = usize::MAX;
            // Calculate the cost of combining weights from i to j
            for k in i..j { // possible split points
                let cost = dp[i][k] + dp[k + 1][j] + weights[i..=j].iter().sum::<usize>();
                dp[i][j] = dp[i][j].min(cost);
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
    println!("{}", result); // Output the result directly
}
