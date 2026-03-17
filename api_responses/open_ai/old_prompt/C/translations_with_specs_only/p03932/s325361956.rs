use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read dimensions
    let dimensions: Vec<usize> = lines
        .next()
        .expect("Input must contain dimensions")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid dimension"))
        .collect();

    let (width, height) = (dimensions[0], dimensions[1]);

    // Ensure dimensions are within the specified limits
    assert!(width > 0 && height > 0 && width <= 205 && height <= 205, "Invalid grid dimensions");

    // Initialize the grid and read values
    let mut grid = vec![vec![0; width]; height];
    for i in 0..height {
        let row: Vec<i32> = lines
            .next()
            .expect("Input must contain grid values")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid grid value"))
            .collect();
        grid[i].copy_from_slice(&row);
    }

    // Initialize the dp array for maximum sums
    let mut dp = vec![vec![0; width]; height];

    // Fill the dp array according to the dynamic programming approach
    for i in 0..height {
        for j in 0..width {
            let current_value = grid[i][j];
            dp[i][j] = current_value;

            if i > 0 {
                dp[i][j] = dp[i][j].max(dp[i - 1][j] + current_value);
            }
            if j > 0 {
                dp[i][j] = dp[i][j].max(dp[i][j - 1] + current_value);
            }
        }
    }

    // The maximum sum collected by two entities is found at the bottom-right corner
    let max_sum = dp[height - 1][width - 1];

    // Output the result
    println!("{}", max_sum);
}
