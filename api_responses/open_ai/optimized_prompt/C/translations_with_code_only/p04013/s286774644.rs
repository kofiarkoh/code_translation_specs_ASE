use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read N and A
    let first_line = input.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    let N = nums.next().unwrap();
    let A = nums.next().unwrap();

    let mut x = vec![0; (N + 1) as usize];
    let mut sum = 0;

    // Read x[i] values
    for i in 1..=N {
        let line = input.next().unwrap().unwrap();
        x[i as usize] = line.parse::<i64>().unwrap();
        sum += x[i as usize];
    }

    // Initialize dp array
    let mut dp = vec![vec![vec![0; (sum + 1) as usize]; (N + 1) as usize]; (N + 1) as usize];

    // Fill dp array
    for j in 0..=N {
        for k in 0..=N {
            for s in 0..=sum {
                if j >= 1 && s < x[j as usize] {
                    dp[j as usize][k as usize][s as usize] = dp[(j - 1) as usize][k as usize][s as usize];
                } else if j >= 1 && k >= 1 && s >= x[j as usize] {
                    dp[j as usize][k as usize][s as usize] = dp[(j - 1) as usize][k as usize][s as usize]
                        + dp[(j - 1) as usize][(k - 1) as usize][(s - x[j as usize]) as usize];
                } else if j == 0 && k == 0 && s == 0 {
                    dp[j as usize][k as usize][s as usize] = 1;
                } else {
                    dp[j as usize][k as usize][s as usize] = 0;
                }
            }
        }
    }

    let mut ans = 0;
    let max = if sum / A < N { sum / A } else { N };

    for k in 1..=max {
        ans += dp[N as usize][k as usize][(k * A) as usize];
    }

    println!("{}", ans);
}
