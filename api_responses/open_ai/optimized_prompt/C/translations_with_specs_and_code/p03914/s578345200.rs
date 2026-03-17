use std::io::{self, Write};

const MOD: i64 = 1_000_000_007;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input for N");
    let m: usize = iter.next().unwrap().parse().expect("Invalid input for M");

    // Preconditions: N and M must be non-negative integers, and N <= 302
    assert!(n <= 302, "N must be less than or equal to 302");
    
    let mut dp = vec![vec![0; 303]; 303]; // Create a 2D array with dimensions (303 x 303)
    dp[1][1] = 1; // Initialize dp[1][1] to 1

    for k in 1..=m {
        for i in (1..=k).rev() {
            for j in (1..=i).rev() {
                let tmp = dp[i][j] % MOD; // Get the current value and apply modulo
                dp[i][j] = 0; // Reset dp[i][j] to 0
                dp[i + 1][j] = (dp[i + 1][j] + tmp * (n as i64 - i as i64)) % MOD; // Update dp[i + 1][j]
                dp[i][i] = (dp[i][i] + tmp * (j as i64)) % MOD; // Update dp[i][i]
                dp[i][j] = (dp[i][j] + tmp * (i as i64 - j as i64)) % MOD; // Update dp[i][j]
            }
        }
    }

    // Output the result
    println!("{}", dp[n][n] % MOD);
    io::stdout().flush().expect("Failed to flush stdout");
}
