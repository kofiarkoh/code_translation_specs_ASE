use std::io::{self, BufRead};

const MAX_SIZE: usize = 100; // Define a maximum size for the grid

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock(); // Locking stdin for reading

    // Read dimensions of the grid
    let mut dimensions = String::new();
    reader.read_line(&mut dimensions).unwrap();
    let dims: Vec<usize> = dimensions.split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    // Ensure we have exactly two dimensions
    if dims.len() != 2 || dims[0] > MAX_SIZE || dims[1] > MAX_SIZE {
        // Behavior is not defined for invalid dimensions
        return;
    }

    let (width, height) = (dims[0], dims[1]);
    let mut grid = vec![vec![0; width]; height];

    // Read grid values
    for i in 0..height {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        grid[i] = line.split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
    }

    // Compute the maximum sum for two entities
    let max_sum = compute_max_sum(&grid, width, height);
    
    // Output the result
    println!("{}", max_sum);
}

// Function to compute the maximum sum collected by two entities
fn compute_max_sum(grid: &[Vec<usize>], width: usize, height: usize) -> usize {
    let mut dp = vec![vec![0; width]; height];

    // Initialize the first cell
    dp[0][0] = grid[0][0];

    // Fill the first row
    for j in 1..width {
        dp[0][j] = dp[0][j - 1] + grid[0][j];
    }

    // Fill the first column
    for i in 1..height {
        dp[i][0] = dp[i - 1][0] + grid[i][0];
    }

    // Fill the rest of the dp table
    for i in 1..height {
        for j in 1..width {
            dp[i][j] = grid[i][j] + dp[i - 1][j].max(dp[i][j - 1]);
        }
    }

    // The maximum sum collected by two entities is in the bottom-right corner
    dp[height - 1][width - 1]
}
