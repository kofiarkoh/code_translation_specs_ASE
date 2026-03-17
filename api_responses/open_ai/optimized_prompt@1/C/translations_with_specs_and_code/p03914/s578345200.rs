fn main() {
    use std::io::{self, BufRead};

    const MOD: i64 = 1_000_000_007;

    // Read input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Initialize the dp array
    let mut dp = vec![vec![0; 302]; 302];
    dp[1][1] = 1;

    // Main computation
    for k in 1..=m {
        for i in (1..=k).rev() {
            for j in (1..=i).rev() {
                let tmp = dp[i][j] % MOD;
                dp[i][j] = 0;
                dp[i + 1][j] = (dp[i + 1][j] + tmp * (n as i64 - i as i64)) % MOD;
                dp[i][i] = (dp[i][i] + tmp * (j as i64)) % MOD;
                dp[i][j] = (dp[i][j] + tmp * (i as i64 - j as i64)) % MOD;
            }
        }
    }

    // Output the result
    println!("{}", dp[n][n] % MOD);
}
