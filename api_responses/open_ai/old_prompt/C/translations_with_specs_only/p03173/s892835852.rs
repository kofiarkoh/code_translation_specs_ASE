fn min_cost_merge(N: usize, a: &[i32]) -> i64 {
    // Initialize the dp array to store minimum costs for subarrays
    let mut dp = vec![vec![0; N]; N];
    // Initialize the sum array to store cumulative sums
    let mut sum = vec![0; N + 1];

    // Compute cumulative sums
    for i in 0..N {
        sum[i + 1] = sum[i] + a[i] as i64; // Store cumulative sums as long long
    }

    // Fill the dp array using dynamic programming
    for length in 2..=N {
        for i in 0..=N - length {
            let j = i + length - 1;
            dp[i][j] = i64::MAX; // Initialize to maximum value

            // Try merging at different points
            for k in i..j {
                let cost = dp[i][k] + dp[k + 1][j] + sum[j + 1] - sum[i];
                dp[i][j] = dp[i][j].min(cost); // Update with the minimum cost
            }
        }
    }

    // Return the minimum cost to merge the entire array
    dp[0][N - 1]
}

fn main() {
    // Example usage
    let N = 4;
    let a = [1, 2, 3, 4];
    let result = min_cost_merge(N, &a);
    println!("{}", result); // Output the result
}
