fn min_cost_merge(N: usize, a: &[i32]) -> i64 {
    // Initialize the dp array to store minimum costs for subarrays
    let mut dp = vec![vec![0; N]; N];
    // Initialize the sum array to store cumulative sums
    let mut sum = vec![0; N + 1];

    // Compute cumulative sums
    for i in 0..N {
        sum[i + 1] = sum[i] + a[i] as i64; // Store cumulative sums as i64
    }

    // Fill the dp array using dynamic programming
    for length in 2..=N { // length of the subarray
        for i in 0..=N - length { // starting index of the subarray
            let j = i + length - 1; // ending index of the subarray
            dp[i][j] = i64::MAX; // Initialize to maximum value
            for k in i..j { // possible split points
                // Calculate cost of merging subarrays and update dp[i][j]
                let cost = dp[i][k] + dp[k + 1][j] + sum[j + 1] - sum[i];
                dp[i][j] = dp[i][j].min(cost);
            }
        }
    }

    // The result for the entire array is stored in dp[0][N-1]
    dp[0][N - 1]
}

fn main() {
    // Example usage
    let N = 4;
    let a = [1, 2, 3, 4];
    let result = min_cost_merge(N, &a);
    println!("{}", result); // Output the minimum cost
}
