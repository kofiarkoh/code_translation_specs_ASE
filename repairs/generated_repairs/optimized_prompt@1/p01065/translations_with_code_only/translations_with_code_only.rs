// AOL 1580 Barter
// 2018.2.18 bal4u

use std::io::{self, BufRead, Read};

const BASE: usize = 300;
const DBASE: usize = 600;
const MAX_N: usize = 101;

fn in_number() -> i32 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut n = 0;
    let mut c = handle.bytes().next().unwrap().unwrap() as char;

    while c.is_digit(10) {
        n = (n << 3) + (n << 1) + (c as i32 - '0' as i32);
        c = handle.bytes().next().unwrap().unwrap() as char;
    }
    n
}

fn main() {
    let n = in_number();
    let x = in_number();
    let y = in_number();
    
    let mut dp = [[[-1; DBASE + 2]; BASE + 2]; MAX_N];
    dp[0][0][BASE] = 0;

    for i in 0..n {
        let a = in_number();
        let b = in_number();
        let c = in_number();
        let d = in_number();

        for j in 0..=BASE {
            for k in 0..=DBASE {
                if dp[i as usize][j][k] < 0 {
                    continue;
                }
                if dp[(i + 1) as usize][j][k] < dp[i as usize][j][k] {
                    dp[(i + 1) as usize][j][k] = dp[i as usize][j][k];
                }
                if x >= j + a && k >= b && dp[(i + 1) as usize][j + a][(k - b) as usize] < dp[i as usize][j][k] {
                    dp[(i + 1) as usize][j + a][(k - b) as usize] = dp[i as usize][j][k];
                }
                if (k + c) <= DBASE && dp[(i + 1) as usize][j][(k + c) as usize] < dp[i as usize][j][k] + d {
                    dp[(i + 1) as usize][j][(k + c) as usize] = dp[i as usize][j][k] + d;
                }
            }
        }
    }

    let mut ans = 0;
    for j in 0..=x {
        for k in 0..=600 {
            if ans < dp[n as usize][j][k] && y + BASE as i32 >= k as i32 {
                ans = dp[n as usize][j][k];
            }
        }
    }
    println!("{}", ans);
}
