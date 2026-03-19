// AOJ 2028: Gather on the Clock
// 2017.11.03 bal4u@uu

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = lines.next().unwrap().unwrap().parse().unwrap();
        }

        let mut dp = vec![vec![0; 102]; 102];

        for k in 1..n {
            for i in 0..n {
                for j in 0..k {
                    let mut x = i + j + 1;
                    if x >= n {
                        x -= n;
                    }
                    let mut v = a[x] - a[i];
                    if v < 0 {
                        v = -v;
                    }
                    v += dp[x][k - j - 1] + dp[i][j];
                    if v > dp[i][k] {
                        dp[i][k] = v;
                    }
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            if dp[i][n - 1] > ans {
                ans = dp[i][n - 1];
            }
        }
        println!("{}", ans);
    }
}
