fn count_combinations(target_sum: usize) -> usize {
    const MODULO: usize = 1_000_000_007; // Specified constant for modulo operation

    // Initialize a vector to hold the count of combinations for each sum
    let mut dp = vec![0; target_sum + 1];
    dp[0] = 1; // Base case: one way to make sum 0 (using no numbers)

    // Iterate through all possible integers that can contribute to the sum
    for i in 1..=target_sum {
        // Update the count of combinations for each sum
        for j in i..=target_sum {
            // Update the count of combinations for the current sum j
            dp[j] = (dp[j] + dp[j - i]) % MODULO;
        }
    }

    // Return the count of valid combinations for the target sum
    dp[target_sum] % MODULO // Ensure the result is within the specified bounds
}

// Example usage
fn main() {
    let target_sum = 123456; // Example input
    let result = count_combinations(target_sum);
    println!("{}", result); // Output the result
}
