use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read N and A
    let first_line = input.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Failed to parse N");
    let a: i64 = parts.next().unwrap().parse().expect("Failed to parse A");

    // Read the sequence of integers
    let mut x = vec![0; n + 1];
    let mut sum = 0;

    for i in 1..=n {
        let line = input.next().unwrap().unwrap();
        x[i] = line.trim().parse().expect("Failed to parse integer");
        sum += x[i];
    }

    // Initialize the dynamic programming table
    let mut dp = vec![vec![vec![0; (sum + 1) as usize]; n + 1]; n + 1];

    // Fill the dynamic programming table
    for j in 0..=n {
        for k in 0..=n {
            for s in 0..=sum {
                if j >= 1 && s < x[j] {
                    dp[j][k][s as usize] = dp[j - 1][k][s as usize];
                } else if j >= 1 && k >= 1 && s >= x[j] {
                    dp[j][k][s as usize] = dp[j - 1][k][s as usize] + dp[j - 1][k - 1][(s - x[j]) as usize];
                } else if j == 0 && k == 0 && s == 0 {
                    dp[j][k][s as usize] = 1;
                } else {
                    dp[j][k][s as usize] = 0;
                }
            }
        }
    }

    let mut ans = 0;
    let max = if sum / a < n as i64 { sum / a } else { n as i64 };

    for k in 1..=max {
        let target_sum = k * a;
        if target_sum <= sum {
            ans += dp[n][k as usize][target_sum as usize];
        }
    }

    println!("{}", ans);
}
