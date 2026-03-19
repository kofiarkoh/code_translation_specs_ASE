const PREDEFINED_CONSTANT: i32 = 1_000_000_007; // Example constant, adjust as needed

fn compute_combinatorial_value(n: usize, m: usize) -> i32 {
    // Preconditions: N and M must be non-negative integers
    // The maximum value for N and M is constrained by the dimensions of the internal data structure.
    // Here we assume a maximum size for the internal structure, for example:
    const MAX_SIZE: usize = 100; // Adjust as necessary

    // Ensure N and M are within bounds
    if n > MAX_SIZE || m > MAX_SIZE {
        panic!("N and M must be less than or equal to {}", MAX_SIZE);
    }

    // Initialize a 2D array to hold computed values
    let mut dp = vec![vec![0; MAX_SIZE + 1]; MAX_SIZE + 1];

    // Base case initialization
    dp[0][0] = 1; // C(0, 0) = 1

    // Fill the 2D array based on combinatorial relationships
    for i in 0..=n {
        for j in 0..=m {
            if j > 0 {
                dp[i][j] = (dp[i][j] + dp[i][j - 1]) % PREDEFINED_CONSTANT; // C(i, j) = C(i, j-1) + C(i-1, j)
            }
            if i > 0 {
                dp[i][j] = (dp[i][j] + dp[i - 1][j]) % PREDEFINED_CONSTANT; // C(i, j) += C(i-1, j)
            }
        }
    }

    // The result is guaranteed to be non-negative and less than the predefined constant
    dp[n][m]
}

fn main() {
    // Read input values for N and M
    let n = 3; // Example input
    let m = 2; // Example input
    let result = compute_combinatorial_value(n, m);
    println!("{}", result); // Output the result directly
}
