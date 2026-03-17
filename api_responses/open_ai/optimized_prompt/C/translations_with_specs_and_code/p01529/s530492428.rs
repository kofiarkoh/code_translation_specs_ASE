use std::io::{self, BufRead};

const MAX_N: usize = 4001;
const INF: i64 = 1 << 60; // A large value to initialize the dp table

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of weights
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input");
    
    // Preconditions check
    if n == 0 {
        panic!("The number of weights must be greater than zero.");
    }

    // Read the weights
    let mut weights = vec![0; n];
    let mut cumulative_weights = vec![0; n];
    let mut total_weight = 0;

    let weight_line = lines.next().unwrap().unwrap();
    let weight_values: Vec<i64> = weight_line.split_whitespace()
        .map(|s| s.parse().expect("Invalid weight"))
        .collect();

    for i in 0..n {
        total_weight += weight_values[i];
        cumulative_weights[i] = total_weight;
        weights[i] = total_weight;
    }

    // Initialize dp and k tables
    let mut dp = vec![vec![INF; n]; n];
    let mut k = vec![vec![0; n]; n];

    for i in 0..n {
        dp[i][i] = 0;
        k[i][i] = i;
    }

    // Dynamic programming to compute minimum cost
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

    // Output the result
    println!("{}", dp[0][n - 1]);
}
