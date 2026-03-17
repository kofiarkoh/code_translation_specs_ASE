use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();

    // Read arrays s and t
    let mut s = vec![0; n as usize];
    let mut t = vec![0; m as usize];
    for i in 0..n {
        s[i as usize] = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    }
    for i in 0..m {
        t[i as usize] = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    }

    // Initialize dp array
    let mut dp = vec![vec![0; 2010]; 2010];

    for i in 0..=n {
        for j in 0..=m {
            if i == 0 || j == 0 {
                dp[i as usize][j as usize] = 1;
                continue;
            }
            dp[i as usize][j as usize] = (dp[(i - 1) as usize][j as usize] + dp[i as usize][(j - 1) as usize]) % MOD;
            if s[(i - 1) as usize] != t[(j - 1) as usize] {
                // Subtract dp[i-1][j-1] in the modular world
                dp[i as usize][j as usize] -= dp[(i - 1) as usize][(j - 1) as usize];
                dp[i as usize][j as usize] = dp[i as usize][j as usize] % MOD;
                if dp[i as usize][j as usize] < 0 {
                    dp[i as usize][j as usize] += MOD;
                }
            }
        }
    }

    // Output the result
    println!("{}", dp[n as usize][m as usize]);
}
