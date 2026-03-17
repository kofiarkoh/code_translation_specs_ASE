// AOJ 2415: Sashimi
// Translated from C to Rust

use std::io::{self, BufRead};

type Ll = i64;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the number of pieces
    let n: usize = handle.lines().next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the weights
    let mut weights = vec![0; n];
    let mut total_weight = 0;
    let mut line = handle.lines().next().unwrap().unwrap();
    let mut iter = line.split_whitespace();

    weights[0] = iter.next().unwrap().parse::<Ll>().unwrap();
    total_weight += weights[0];

    for i in 1..n {
        let weight: Ll = iter.next().unwrap().parse().unwrap();
        total_weight += weight;
        weights[i] = total_weight;
    }

    // Initialize dp and k arrays
    let mut dp = vec![vec![std::i64::MAX; n]; n];
    let mut k = vec![vec![0; n]; n];

    for i in 0..n {
        dp[i][i] = 0;
        k[i][i] = i;
    }

    // Dynamic programming to calculate minimum cost
    for a in 1..=n {
        for i in 0..n - a + 1 {
            let j = i + a - 1;
            for b in k[i][j - 1]..=k[i + 1][j] {
                let mut cost = dp[i][b] + dp[b + 1][j] + weights[j];
                if i > 0 {
                    cost -= weights[i - 1];
                }
                if dp[i][j] > cost {
                    dp[i][j] = cost;
                    k[i][j] = b;
                }
            }
        }
    }

    // Output the minimum cost
    println!("{}", dp[0][n - 1]);
}
