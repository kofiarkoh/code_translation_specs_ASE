use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read lengths of sequences
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Read sequences
    let mut s = vec![0; n];
    let mut t = vec![0; m];
    
    let second_line = lines.next().unwrap().unwrap();
    s.iter_mut().zip(second_line.split_whitespace()).for_each(|(x, y)| *x = y.parse().unwrap());
    
    let third_line = lines.next().unwrap().unwrap();
    t.iter_mut().zip(third_line.split_whitespace()).for_each(|(x, y)| *x = y.parse().unwrap());

    // Initialize the dynamic programming table
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Fill the dynamic programming table
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
