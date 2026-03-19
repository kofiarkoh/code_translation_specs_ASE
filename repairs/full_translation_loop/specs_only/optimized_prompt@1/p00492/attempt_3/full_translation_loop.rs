use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read width and height
    let first_line = lines.next().unwrap_or_else(|_| panic!("Failed to read dimensions."));
    let dimensions: Vec<usize> = first_line.unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Ensure we have exactly two dimensions
    assert!(dimensions.len() == 2, "Expected exactly two dimensions.");
    let width = dimensions[0];
    let height = dimensions[1];

    // Preconditions: width and height must be positive integers
    assert!(width > 0 && height > 0, "Width and height must be positive integers.");

    // Initialize the grid with dimensions (height + 2) x (width * 2 + 4)
    let grid_height = height + 2;
    let grid_width = width * 2 + 4;
    let mut grid = vec![vec![0; grid_width]; grid_height];

    // Read the grid values
    for i in 0..grid_height {
        let line = lines.next().unwrap_or_else(|_| panic!("Failed to read grid row {}.", i));
        let values: Vec<i32> = line.unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        // Ensure we do not exceed the grid width
        for j in 0..grid_width {
            if j < values.len() {
                grid[i][j] = values[j];
            }
        }
    }

    // Mark cells based on movement rules
    let mut marked = vec![vec![false; grid_width]; grid_height];
    for i in 1..grid_height - 1 {
        for j in 1..grid_width - 1 {
            // Example marking logic (to be defined based on specific rules)
            if grid[i][j] > 0 { // Placeholder condition for marking
                marked[i][j] = true;
            }
        }
    }

    // Count unmarked cells adjacent to marked cells
    let mut count = 0;
    for i in 1..grid_height - 1 {
        for j in 1..grid_width - 1 {
            if !marked[i][j] {
                // Check adjacent cells
                if marked[i - 1][j] || marked[i + 1][j] || marked[i][j - 1] || marked[i][j + 1] {
                    count += 1;
                }
            }
        }
    }

    // Output the count
    println!("{}", count);
}
