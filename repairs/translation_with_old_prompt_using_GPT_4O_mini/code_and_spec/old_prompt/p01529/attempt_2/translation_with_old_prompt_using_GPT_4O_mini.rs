use std::io::{self, BufRead};

const MAX_N: usize = 4001;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the number of pieces
    let mut lines = handle.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the weights
    let mut weights = vec![0; n];
    let weight_input: Vec<u64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    weights[0] = weight_input[0];
    for i in 1..n {
        weights[i] = weights[i - 1] + weight_input[i];
    }

    // Initialize dp and k arrays
    let mut dp = vec![vec![u64::MAX; MAX_N]; MAX_N];
    let mut k = vec![vec![0; MAX_N]; MAX_N];

    for i in 0..n {
        dp[i][i] = 0;
        k[i][i] = i;
    }

    // Dynamic programming to calculate minimum cost
    for a in 1..=n {
        for i in 0..n - a + 1 {
            let j = i + a - 1;
            for b in k[i][j - 1]..=k[i + 1][j] {
                if b >= n { continue; } // Prevent out-of-bounds access
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

    // Output the result
    println!("{}", dp[0][n - 1]);
}
