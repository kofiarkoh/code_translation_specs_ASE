// AOJ 2488 Tree Construction
// 2018.2.28 bal4u

use std::io::{self, BufRead};

const ABS: fn(i32) -> i32 = |a| if a >= 0 { a } else { -a };
const MAX_SIZE: usize = 1001;

fn in_number<R: BufRead>(reader: &mut R) -> i32 {
    let mut n = 0;
    let mut c = reader.bytes().next().unwrap().unwrap();
    while c >= b'0' {
        n = 10 * n + (c - b'0') as i32;
        c = reader.bytes().next().unwrap().unwrap();
    }
    n
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let n = in_number(&mut reader);
    let mut x = vec![0; MAX_SIZE];
    let mut y = vec![0; MAX_SIZE];
    let mut dp = vec![vec![0x10; MAX_SIZE]; MAX_SIZE];

    for i in 0..n {
        x[i as usize] = in_number(&mut reader);
        y[i as usize] = in_number(&mut reader);
    }

    for i in 0..n {
        dp[i as usize][i as usize] = 0;
    }

    for l in 0..n {
        for i in 0..n {
            if i + l < n {
                for j in i..(i + l) {
                    let t = dp[i as usize][j as usize]
                        + dp[j as usize + 1][i as usize + l]
                        + ABS(x[i as usize] - x[j as usize + 1])
                        + ABS(y[j as usize] - y[i as usize + l]);
                    if t < dp[i as usize][(i + l) as usize] {
                        dp[i as usize][(i + l) as usize] = t;
                    }
                }
            }
        }
    }

    println!("{}", dp[0][(n - 1) as usize]);
}
