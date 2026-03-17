fn min_cut_cost(weights: Vec<u64>) -> u64 {
    let n = weights.len();
    
    // Preconditions: number of weights must be greater than zero
    if n == 0 {
        panic!("Number of weights must be greater than zero.");
    }

    // Initialize the dynamic programming table
    let mut dp = vec![vec![0; n]; n];

    // Fill the dp table
    for length in 1..=n {
        for i in 0..=n - length {
            let j = i + length - 1;
            // Base case: no cuts needed for a single segment
            if length == 1 {
                dp[i][j] = 0; // No cost for a single segment
            } else {
                // Calculate the cost of cutting at each possible position
                for k in i..j {
                    let cost = dp[i][k] + dp[k + 1][j] + weights[i..=j].iter().sum::<u64>();
                    dp[i][j] = dp[i][j].min(cost);
                }
            }
        }
    }

    // The minimum cost to cut the entire rod is stored in dp[0][n-1]
    dp[0][n - 1]
}

fn main() {
    // Example usage
    let weights = vec![1, 2, 3, 4]; // Example weights
    let min_cost = min_cut_cost(weights);
    println!("Minimum cost of cutting the rod: {}", min_cost);
}
