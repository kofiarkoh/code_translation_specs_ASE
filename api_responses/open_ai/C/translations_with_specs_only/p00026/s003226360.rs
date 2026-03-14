use std::io::{self, BufRead};

fn main() {
    // Initialize a 10x10 grid with all cells set to 0 (untainted)
    let mut grid = [[0; 10]; 10];

    // Read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Unwrap the line to get the input string
        let line = line.unwrap();
        
        // Split the line by commas and parse the integers
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            continue; // Skip invalid input formats
        }

        // Parse x, y, and z coordinates
        let x: usize = parts[0].parse().unwrap_or(10); // Default to 10 to skip out of range
        let y: usize = parts[1].parse().unwrap_or(10); // Default to 10 to skip out of range
        let z: usize = parts[2].parse().unwrap_or(0); // Default to 0 for invalid taint level

        // Check if x and y are within the valid range (0 to 9)
        if x < 10 && y < 10 && z >= 1 && z <= 3 {
            // Increment the taint level at the specified coordinates
            grid[x][y] += z;
        }
    }

    // Calculate the number of untainted cells and the maximum taint level
    let mut untainted_count = 0;
    let mut max_taint_level = 0;

    for row in &grid {
        for &cell in row {
            if cell == 0 {
                untainted_count += 1; // Count untainted cells
            }
            if cell > max_taint_level {
                max_taint_level = cell; // Update maximum taint level
            }
        }
    }

    // Output the results
    println!("{} {}", untainted_count, max_taint_level);
}
