fn main() {
    use std::io::{self, BufRead};

    // Read input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of pieces
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the weights
    let weights: Vec<u64> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Edge case: if there's only one piece, no cuts are needed
    if n == 1 {
        println!("0");
        return;
    }

    // Initialize dp and k arrays
    let mut dp = vec![vec![0; n]; n];
    let mut k = vec![vec![0; n]; n];

    // Precompute the sum of weights for segments
    let mut sum = vec![vec![0; n]; n];
    for i in 0..n {
        sum[i][i] = weights[i];
        for j in i + 1..n {
            sum[i][j] = sum[i][j - 1] + weights[j];
        }
    }

    // Dynamic programming to calculate minimum cost
    for length in 2..=n {
        for i in 0..=n - length {
            let j = i + length - 1;
            dp[i][j] = u64::MAX; // Initialize to maximum value
            for cut in i..j {
                let cost = dp[i][cut] + dp[cut + 1][j] + sum[i][j];
                if cost < dp[i][j] {
                    dp[i][j] = cost;
                    k[i][j] = cut; // Store the cut position
                }
            }
        }
    }

    // The result is the minimum cost to cut the whole fish
    println!("{}", dp[0][n - 1]);
}
