// AOJ 2488 Tree Construction
// 2018.2.28 bal4u

use std::io::{self, Read};

const ABS: fn(i32) -> i32 = |a| if a >= 0 { a } else { -a };
const MAX_N: usize = 1001;

fn in_number() -> i32 {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut n = 0;
    let mut c = handle.by_ref().bytes().next().unwrap().unwrap() as char;

    while c.is_digit(10) {
        n = 10 * n + (c as i32 - '0' as i32);
        c = handle.by_ref().bytes().next().unwrap().unwrap() as char;
    }
    n
}

fn main() {
    let n = in_number();
    let mut x = vec![0; MAX_N];
    let mut y = vec![0; MAX_N];
    let mut dp = vec![vec![0x10; MAX_N]; MAX_N];

    for i in 0..n {
        x[i as usize] = in_number();
        y[i as usize] = in_number();
    }

    for i in 0..n {
        dp[i as usize][i as usize] = 0;
    }

    for l in 0..n {
        for i in 0..n {
            if i + l < n {
                for j in i..(i + l) {
                    let t = dp[i as usize][j as usize] + dp[(j + 1) as usize][(i + l) as usize] + 
                            ABS(x[i as usize] - x[(j + 1) as usize]) + ABS(y[j as usize] - y[(i + l) as usize]);
                    if t < dp[i as usize][(i + l) as usize] {
                        dp[i as usize][(i + l) as usize] = t;
                    }
                }
            }
        }
    }
    println!("{}", dp[0][(n - 1) as usize]);
}
