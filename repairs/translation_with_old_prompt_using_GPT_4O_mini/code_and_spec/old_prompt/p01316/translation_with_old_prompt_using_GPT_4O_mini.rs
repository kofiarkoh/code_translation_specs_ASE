use std::io::{self, BufRead};
use std::cmp::max;

const MAX_DP: usize = 20001;
const MAX_C: usize = 16;
const DP_SIZE: usize = 256;

fn min(x: i32, y: i32) -> i32 {
    if x < y { x } else { y }
}

fn f(x: i32, y: i32) -> i32 {
    (x - y) * (x - y)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut dp = [[-1; DP_SIZE]; MAX_DP];
    let mut C = [0; MAX_C];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 {
            break; // Terminate if n is zero
        }

        for i in 0..m {
            C[i] = iter.next().unwrap().parse().unwrap();
        }

        dp[0][128] = 0; // Initialize dp[0][128] to 0

        for i in 0..n {
            let x: i32 = iter.next().unwrap().parse().unwrap();
            let prev = dp[i];
            let curr = &mut dp[i + 1];

            for j in 0..DP_SIZE {
                if prev[j] >= 0 {
                    for k in 0..m {
                        let t = min(255, max(0, j as i32 + C[k]));
                        let y = prev[j] + f(x, t);
                        curr[t] = if curr[t] < 0 { y } else { min(curr[t], y) };
                    }
                }
            }
        }

        let mut res = i32::MAX;
        for i in 0..DP_SIZE {
            if dp[n][i] >= 0 { // Use dp[n] instead of curr
                res = min(res, dp[n][i]);
            }
        }
        println!("{}", res);
    }
}
