// This program computes the minimum cost of partitioning a sequence of weights into contiguous segments.
// The cost of each segment is determined by the sum of its weights.

fn min_partition_cost(n: usize, weights: &[i32]) -> i32 {
    // Preconditions: n must be greater than zero
    if n == 0 {
        panic!("The number of weights must be greater than zero.");
    }

    // Initialize a dynamic programming table with a high value
    let mut dp = vec![i32::MAX; n + 1];
    dp[0] = 0; // Base case: cost of partitioning an empty sequence is 0

    // Iterate over each possible end of the segment
    for i in 1..=n {
        let mut segment_sum = 0;

        // Calculate the cost for segments ending at i
        for j in (1..=i).rev() {
            segment_sum += weights[j - 1]; // Sum the weights for the current segment
            // Ensure the computed cost is non-negative
            if segment_sum < 0 {
                segment_sum = 0; // Reset if negative, though this should not happen with valid input
            }
            // Update the dp table with the minimum cost
            dp[i] = dp[i].min(dp[j - 1] + segment_sum);
        }
    }

    // The minimum cost of partitioning the weights is stored in dp[n]
    dp[n]
}

// Example usage
fn main() {
    let weights = vec![1, 2, 3, 4]; // Example weights
    let n = weights.len(); // Number of weights
    let min_cost = min_partition_cost(n, &weights);
    println!("Minimum cost of partitioning: {}", min_cost);
}
