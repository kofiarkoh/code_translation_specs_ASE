fn main() {
    // Constants
    const MOD: i64 = 1_000_000_007;

    // Read inputs for N and M
    let (n, m) = read_input();

    // Initialize the dynamic programming table
    let mut dp = vec![vec![0; (m + 1) as usize]; (n + 1) as usize];

    // Base case: There is 1 way to distribute 0 items into any number of boxes
    for i in 0..=n {
        dp[i][0] = 1;
    }

    // Fill the dynamic programming table
    for i in 1..=n {
        for j in 1..=m {
            // dp[i][j] = dp[i-1][j] + dp[i][j-1]
            dp[i][j] = (dp[i - 1][j] + dp[i][j - 1]) % MOD;
        }
    }

    // The result is in dp[n][m]
    println!("{}", dp[n][m]);
}

// Function to read input values for N and M
fn read_input() -> (i64, i64) {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line and parse N and M
    let first_line = lines.next().unwrap().unwrap();
    let nums: Vec<i64> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    (nums[0], nums[1])
}
