fn count_valid_subsets(n: usize, a: i64, x: &[i64]) -> i64 {
    // Check preconditions
    if n == 0 || a <= 0 {
        return 0; // If N is 0 or A is not positive, return 0 as per specification
    }

    // Calculate the total sum of the integers
    let total_sum: i64 = x.iter().sum();
    
    // Initialize the dynamic programming table
    // dp[j][k] will hold the count of subsets of the first j elements
    // that have a sum equivalent to k modulo A
    let mut dp = vec![vec![0; a as usize]; n + 1];
    dp[0][0] = 1; // Base case: one way to select an empty subset with sum 0

    // Fill the dynamic programming table
    for i in 0..n {
        for j in 0..=i {
            for k in 0..a {
                // Carry forward the previous counts
                dp[j + 1][(k + (x[i] % a + a) % a) as usize % a] += dp[j][k as usize];
                dp[j][k as usize] += dp[j][k as usize]; // Include the case of not taking x[i]
            }
        }
    }

    // Sum up all valid subsets whose sums are multiples of A
    let mut result = 0;
    for j in 1..=n {
        result += dp[j][0]; // Count subsets where sum % A == 0
    }

    result
}

fn main() {
    // Example usage
    let n: usize = 3; // Number of integers
    let a: i64 = 5; // The integer that the sum must be a multiple of
    let x: Vec<i64> = vec![1, 2, 3]; // The list of integers

    let count = count_valid_subsets(n, a, &x);
    println!("{}", count); // Output the result
}
