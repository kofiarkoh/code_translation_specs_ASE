// AOJ 2488 Tree Construction
// 2018.2.28 bal4u

use std::io::{self, BufRead};

const ABS: fn(i32) -> i32 = |a| if a >= 0 { a } else { -a };
const N: usize = 1001;

fn in_number() -> i32 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut n = 0;
    let mut c = handle.bytes().next().unwrap().unwrap() as char;

    while c >= '0' {
        n = 10 * n + (c as i32 - '0' as i32);
        c = handle.bytes().next().unwrap().unwrap() as char;
    }
    n
}

fn main() {
    let n = in_number();
    let mut x = vec![0; N];
    let mut y = vec![0; N];
    let mut dp = vec![vec![0x10; N]; N];

    for i in 0..n {
        x[i] = in_number();
        y[i] = in_number();
    }

    for i in 0..n {
        dp[i][i] = 0;
    }

    for l in 0..n {
        for i in 0..n {
            if i + l < n {
                for j in i..(i + l) {
                    let t = dp[i][j] + dp[j + 1][i + l] + ABS(x[i] - x[j + 1]) + ABS(y[j] - y[i + l]);
                    if t < dp[i][i + l] {
                        dp[i][i + l] = t;
                    }
                }
            }
        }
    }
    println!("{}", dp[0][(n - 1) as usize]);
}
