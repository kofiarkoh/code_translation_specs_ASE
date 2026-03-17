// AOJ 2415: Sashimi
// 2017.12.8 bal4u@uu

use std::io::{self, BufRead};

const MAX_N: usize = 4001;

fn get_long(p: &mut &[u8]) -> i64 {
    let mut n = 0;
    while *p.first().unwrap_or(&b'\0') >= b'0' {
        n = (n << 3) + (n << 1) + (*p.first().unwrap() - b'0') as i64;
        *p = &p[1..];
    }
    *p = &p[1..]; // Move past the non-digit character
    n
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut buf = vec![0; 48002];
    let n: usize = reader.read_line(&mut buf).unwrap().trim().parse().unwrap();
    
    let mut p = &buf[..];
    let mut w = vec![0; MAX_N];
    w[0] = get_long(&mut p);
    
    for i in 1..n {
        w[i] = w[i - 1] + get_long(&mut p);
    }

    let mut dp = vec![vec![0x3f3f3f3f3f3f3f3f; MAX_N]; MAX_N];
    let mut k = vec![vec![0; MAX_N]; MAX_N];

    for i in 0..n {
        dp[i][i] = 0;
        k[i][i] = i;
    }

    for a in 1..=n {
        for i in 0..n - a + 1 {
            let j = i + a - 1;
            for b in k[i][j - 1]..=k[i + 1][j] {
                let mut c = dp[i][b] + dp[b + 1][j] + w[j];
                if i > 0 {
                    c -= w[i - 1];
                }
                if dp[i][j] > c {
                    dp[i][j] = c;
                    k[i][j] = b;
                }
            }
        }
    }

    println!("{}", dp[0][n - 1]);
}
