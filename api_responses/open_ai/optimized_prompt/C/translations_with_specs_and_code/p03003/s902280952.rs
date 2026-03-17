use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();

    // Read arrays s and t
    let mut s = Vec::with_capacity(n);
    let mut t = Vec::with_capacity(m);
    
    let s_line = lines.next().unwrap().unwrap();
    s.extend(s_line.split_whitespace().map(|s| s.parse::<i64>().unwrap()));
    
    let t_line = lines.next().unwrap().unwrap();
    t.extend(t_line.split_whitespace().map(|s| s.parse::<i64>().unwrap()));

    // Initialize the dp array
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Fill the dp array according to the specification
    for i in 0..=n {
        for j in 0..=m {
            if i == 0 || j == 0 {
                dp[i][j] = 1;
                continue;
            }
            dp[i][j] = (dp[i - 1][j] + dp[i][j - 1]) % MOD;
            if s[i - 1] != t[j - 1] {
                dp[i][j] = (dp[i][j] - dp[i - 1][j - 1] + MOD) % MOD; // Ensure non-negative
            }
        }
    }

    // Output the result
    println!("{}", dp[n][m]);
}
