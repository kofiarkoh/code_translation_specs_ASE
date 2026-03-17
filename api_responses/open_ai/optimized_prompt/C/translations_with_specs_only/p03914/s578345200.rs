// Define the constant for the modulo operation
const MODULO_CONSTANT: i32 = 1_000_000_007;

// Function to compute the combinatorial value based on inputs N and M
fn compute_combinatorial_value(n: usize, m: usize) -> i32 {
    // Preconditions: N and M must be non-negative integers
    // The maximum value for N is constrained to 302
    assert!(n <= 302, "N must be less than or equal to 302");
    
    // Create a two-dimensional array to hold the computed values
    // The size is (N+1) x (M+1) to accommodate 0-based indexing
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Initialize the base case
    dp[0][0] = 1; // There is one way to choose 0 items from 0 items

    // Fill the two-dimensional array based on combinatorial rules
    for i in 1..=n {
        for j in 0..=m {
            // Update the dp array based on previous values
            dp[i][j] = dp[i - 1][j]; // Not choosing the i-th item
            if j > 0 {
                dp[i][j] += dp[i - 1][j - 1]; // Choosing the i-th item
            }
            // Apply modulo to keep values non-negative and within bounds
            dp[i][j] %= MODULO_CONSTANT;
        }
    }

    // The result is found at dp[n][m], which is guaranteed to be non-negative
    dp[n][m]
}

// Example usage (uncomment to test)
// fn main() {
//     let n = 5;
//     let m = 3;
//     let result = compute_combinatorial_value(n, m);
//     println!("Result: {}", result);
// }
