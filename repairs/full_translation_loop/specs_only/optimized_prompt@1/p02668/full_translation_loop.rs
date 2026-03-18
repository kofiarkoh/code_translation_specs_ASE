fn combinatorial_result(n: usize, m: usize) -> i32 {
    // Precondition check: n and m must be non-negative
    if n < 0 || m < 0 {
        return -1; // Invalid input
    }

    // Define a constant for modulo operation
    const MODULO: i32 = 1_000_000_007;

    // If n or m is out of bounds, return -1
    if n > 1000 || m > 1000 { // Assuming 1000 is the maximum size constraint
        return -1;
    }

    // Initialize a dynamic programming table
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Base case: C(0, 0) = 1
    dp[0][0] = 1;

    // Fill the dynamic programming table
    for i in 0..=n {
        for j in 0..=m {
            if j > i {
                dp[i][j] = 0; // Cannot choose more than available
            } else if j == 0 {
                dp[i][j] = 1; // There's one way to choose 0 items
            } else {
                // C(n, m) = C(n-1, m-1) + C(n-1, m)
                dp[i][j] = (dp[i - 1][j - 1] + dp[i - 1][j]) % MODULO;
            }
        }
    }

    // Return the result from the DP table
    dp[n][m]
}

fn main() {
    // Example usage of the combinatorial_result function
    let n = 5;
    let m = 3; // Adjusted to match the test case
    let result = combinatorial_result(n, m);
    println!("{}", result); // Output the result
}
