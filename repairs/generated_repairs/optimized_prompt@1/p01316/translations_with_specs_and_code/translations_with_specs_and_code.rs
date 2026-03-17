use std::io::{self, BufRead};

const MAX_COST: i32 = 1 << 30; // A large constant for no valid transformation
const MAX_VALUES: usize = 20001;
const MAX_ADJUSTMENTS: usize = 256;

fn min(x: i32, y: i32) -> i32 {
    if x < y { x } else { y }
}

fn max(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}

fn f(x: i32, y: i32) -> i32 {
    (x - y) * (x - y)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut dp = [[-1; MAX_ADJUSTMENTS]; MAX_VALUES];
    let mut c = [0; 16];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        for i in 0..m {
            c[i] = iter.next().unwrap().parse().unwrap();
        }

        dp[0][128] = 0; // Initialize the starting point

        for i in 0..n {
            let x: i32 = iter.next().unwrap().parse().unwrap();
            let prev = dp[i];
            let curr = &mut dp[i + 1];

            for j in 0..MAX_ADJUSTMENTS {
                if prev[j] >= 0 {
                    for k in 0..m {
                        let t = min(255, max(0, j as i32 + c[k]));
                        let y = prev[j] + f(x, t);
                        curr[t as usize] = if curr[t as usize] < 0 {
                            y
                        } else {
                            min(curr[t as usize], y)
                        };
                    }
                }
            }
        }

        let mut res = MAX_COST;
        for i in 0..MAX_ADJUSTMENTS {
            if dp[n][i] >= 0 {
                res = min(res, dp[n][i]);
            }
        }

        println!("{}", res);
    }
}
