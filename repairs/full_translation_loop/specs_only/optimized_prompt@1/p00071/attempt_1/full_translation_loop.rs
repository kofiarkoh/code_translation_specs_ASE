use std::io::{self, BufRead};

const GRID_SIZE: usize = 10;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let test_cases: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..test_cases {
        // Initialize the grid
        let mut grid: [[char; GRID_SIZE]; GRID_SIZE] = [[' '; GRID_SIZE]; GRID_SIZE];

        // Read the grid
        for i in 0..GRID_SIZE {
            let line = lines.next().unwrap().unwrap();
            for (j, c) in line.chars().enumerate() {
                grid[i][j] = c;
            }
        }

        // Read the starting coordinates for the explosion
        let coords: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let start_x = coords[0];
        let start_y = coords[1];

        // Check if the starting coordinates are within bounds
        if start_x < GRID_SIZE && start_y < GRID_SIZE {
            let target_value = grid[start_x][start_y];
            let replacement_value = '*'; // Assuming '*' indicates the explosion effect

            // Perform the explosion effect
            explode(&mut grid, start_x, start_y, target_value, replacement_value);
        }

        // Print the modified grid
        for row in grid.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

// Function to perform the explosion effect using DFS
fn explode(grid: &mut [[char; GRID_SIZE]; GRID_SIZE], x: usize, y: usize, target: char, replacement: char) {
    // Check bounds and if the current cell matches the target value
    if x >= GRID_SIZE || y >= GRID_SIZE || grid[x][y] != target {
        return;
    }

    // Replace the current cell with the replacement value
    grid[x][y] = replacement;

    // Recursively explode in all four directions
    explode(grid, x.wrapping_sub(1), y, target, replacement); // Up
    explode(grid, x + 1, y, target, replacement); // Down
    explode(grid, x, y.wrapping_sub(1), target, replacement); // Left
    explode(grid, x, y + 1, target, replacement); // Right
}
