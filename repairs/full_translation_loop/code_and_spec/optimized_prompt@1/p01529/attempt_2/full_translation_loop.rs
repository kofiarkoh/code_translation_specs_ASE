use std::io::{self, BufRead};

const MAX_SIZE: usize = 4001;

fn get_long(p: &mut std::str::Chars) -> i64 {
    let mut n = 0;
    while let Some(c) = p.next() {
        if c.is_digit(10) {
            n = (n << 3) + (n << 1) + (c as i64 - '0' as i64);
        } else {
            break;
        }
    }
    n
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the number of weights
    let mut buf = String::new();
    handle.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().expect("Invalid input for number of weights");

    if n == 0 {
        // Handle the case where the number of weights is zero
        println!("0");
        return;
    }

    // Read the weights
    buf.clear();
    handle.read_line(&mut buf).unwrap();
    let mut weights = vec![0; n];
    let mut p = buf.chars();
    weights[0] = get_long(&mut p);
    for i in 1..n {
        weights[i] = weights[i - 1] + get_long(&mut p);
    }

    // Initialize the dynamic programming table
    let mut dp = vec![vec![i64::MAX; MAX_SIZE]; MAX_SIZE];
    let mut k = vec![vec![0; MAX_SIZE]; MAX_SIZE];

    for i in 0..n {
        dp[i][i] = 0;
        k[i][i] = i;
    }

    // Fill the dynamic programming table
    for a in 1..=n {
        for i in 0..n - a + 1 {
            let j = i + a - 1;
            for b in k[i][j - 1]..=k[i + 1][j] {
                let mut c = dp[i][b] + dp[b + 1][j] + weights[j];
                if i > 0 {
                    c -= weights[i - 1];
                }
                if dp[i][j] > c {
                    dp[i][j] = c;
                    k[i][j] = b;
                }
            }
        }
    }

    // Output the result
    println!("{}", dp[0][n - 1]);
}
