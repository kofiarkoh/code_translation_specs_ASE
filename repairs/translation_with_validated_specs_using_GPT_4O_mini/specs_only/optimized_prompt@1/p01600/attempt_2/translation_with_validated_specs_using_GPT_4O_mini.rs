fn min_cost_to_connect_points(n: usize, x: Vec<i32>, y: Vec<i32>) -> i32 {
    // Initialize a dynamic programming table with high values
    let mut dp = vec![vec![i32::MAX; n]; n];

    // The cost of connecting a single point to itself is zero
    for i in 0..n {
        dp[i][i] = 0;
    }

    // Iterate over the lengths of segments
    for length in 1..n {
        for i in 0..n - length {
            let j = i + length;
            // Calculate the minimum cost to connect points from i to j
            for k in i..j {
                let cost = (x[j] - x[i]).abs() + (y[j] - y[i]).abs();
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j] + cost);
            }
        }
    }

    // The minimum cost to connect all points is found in dp[0][n-1]
    dp[0][n - 1]
}

fn main() {
    // Example usage
    let n = 4;
    let x = vec![0, 1, 2, 3];
    let y = vec![0, 1, 2, 3];
    let result = min_cost_to_connect_points(n, x, y);
    println!("Minimum cost to connect points: {}", result);
}
