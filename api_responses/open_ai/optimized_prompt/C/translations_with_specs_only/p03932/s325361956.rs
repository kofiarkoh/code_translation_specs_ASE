use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line for grid dimensions
    let dimensions: Vec<usize> = reader.lines()
        .next()
        .expect("Failed to read dimensions")
        .expect("Invalid input")
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid dimension"))
        .collect();

    let (width, height) = (dimensions[0], dimensions[1]);

    // Initialize the grid
    let mut grid = vec![vec![0; width]; height];

    // Read the grid values
    for i in 0..height {
        let line: Vec<i32> = reader.lines()
            .nth(i)
            .expect("Failed to read grid row")
            .expect("Invalid input")
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid grid value"))
            .collect();
        grid[i] = line;
    }

    // Compute the maximum sum collected by two entities
    let max_sum = compute_max_sum(&grid);
    
    // Output the result
    println!("{}", max_sum);
}

// Function to compute the maximum sum of values collected by two entities
fn compute_max_sum(grid: &[Vec<i32>]) -> i32 {
    let height = grid.len();
    let width = grid[0].len();

    // Create a DP table to store maximum sums
    let mut dp = vec![vec![0; width]; height];

    // Initialize the DP table with the first cell
    dp[0][0] = grid[0][0];

    // Fill the first row
    for j in 1..width {
        dp[0][j] = dp[0][j - 1] + grid[0][j];
    }

    // Fill the first column
    for i in 1..height {
        dp[i][0] = dp[i - 1][0] + grid[i][0];
    }

    // Fill the rest of the DP table
    for i in 1..height {
        for j in 1..width {
            dp[i][j] = grid[i][j] + dp[i - 1][j].max(dp[i][j - 1]);
        }
    }

    // The maximum sum collected by one entity is in the bottom-right corner
    let max_sum_entity1 = dp[height - 1][width - 1];

    // To simulate two entities, we can use a second DP table
    let mut dp2 = vec![vec![0; width]; height];
    dp2[0][0] = grid[0][0];

    // Fill the second DP table similarly
    for j in 1..width {
        dp2[0][j] = dp2[0][j - 1] + grid[0][j];
    }

    for i in 1..height {
        dp2[i][0] = dp2[i - 1][0] + grid[i][0];
    }

    for i in 1..height {
        for j in 1..width {
            dp2[i][j] = grid[i][j] + dp2[i - 1][j].max(dp2[i][j - 1]);
        }
    }

    // The maximum sum collected by the second entity
    let max_sum_entity2 = dp2[height - 1][width - 1];

    // Return the total maximum sum collected by both entities
    max_sum_entity1 + max_sum_entity2
}
