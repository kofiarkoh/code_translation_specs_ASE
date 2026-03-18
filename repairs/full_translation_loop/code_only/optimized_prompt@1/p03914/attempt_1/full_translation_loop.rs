use std::io::{self, Write};

const MOD: i64 = 1_000_000_007;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut dp = vec![vec![0; 302]; 302];
    dp[1][1] = 1;

    for k in 1..=m {
        for i in (1..=k).rev() {
            for j in (1..=i).rev() {
                let tmp = dp[i][j] % MOD;
                dp[i][j] = 0;
                dp[i + 1][j] = (dp[i + 1][j] + tmp * (n as i64 - i as i64)) % MOD;
                dp[i][i] = (dp[i][i] + tmp * j as i64) % MOD;
                dp[i][j] = (dp[i][j] + tmp * (i as i64 - j as i64)) % MOD;
            }
        }
    }

    println!("{}", dp[n][n] % MOD);
    io::stdout().flush().unwrap();
}
