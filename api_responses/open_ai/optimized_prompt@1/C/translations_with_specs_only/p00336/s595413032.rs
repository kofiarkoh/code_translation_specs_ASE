use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first string (target subsequence)
    let target = lines.next().unwrap().unwrap();
    // Read the second string (source string)
    let source = lines.next().unwrap().unwrap();

    // Define the modulo prime number
    const MOD: i64 = 1_000_000_007;

    // Ensure the lengths of the strings do not exceed 1000 characters
    if target.len() > 1000 || source.len() > 1000 {
        // Behavior is not defined if input exceeds maximum length
        return;
    }

    // Get lengths of the strings
    let n = target.len();
    let m = source.len();

    // Create a 2D array to store the count of distinct subsequences
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Base case: An empty target string has one subsequence (the empty subsequence)
    for j in 0..=m {
        dp[0][j] = 1;
    }

    // Fill the dp array
    for i in 1..=n {
        for j in 1..=m {
            // If characters match, we can either include or exclude the character
            if target.chars().nth(i - 1) == source.chars().nth(j - 1) {
                dp[i][j] = (dp[i - 1][j - 1] + dp[i][j - 1]) % MOD;
            } else {
                // If characters do not match, we can only exclude the character from source
                dp[i][j] = dp[i][j - 1] % MOD;
            }
        }
    }

    // The result is the count of distinct subsequences of target in source
    let result = dp[n][m];

    // Output the result
    println!("{}", result);
}
