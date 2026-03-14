use std::io::{self, BufRead};

const GRID_SIZE: usize = 12;

fn delete_island(grid: &mut [[u8; GRID_SIZE + 2]; GRID_SIZE], x: isize, y: isize) {
    // Check bounds and if the current cell is land
    if x < 0 || x >= GRID_SIZE as isize || y < 0 || y >= GRID_SIZE as isize || grid[x as usize][y as usize] == 0 {
        return;
    }
    // Mark the current cell as water
    grid[x as usize][y as usize] = 0;
    // Recursively delete adjacent land cells
    delete_island(grid, x - 1, y);
    delete_island(grid, x + 1, y);
    delete_island(grid, x, y - 1);
    delete_island(grid, x, y + 1);
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut grid = [[0; GRID_SIZE + 2]; GRID_SIZE]; // Create a grid with padding for easier boundary checks

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        // Read the first row into the grid
        grid[0][1..=GRID_SIZE].copy_from_slice(&line.as_bytes()[0..GRID_SIZE]);
        
        // Read the remaining rows
        for i in 1..GRID_SIZE {
            let line = reader.lines().next().expect("Failed to read line").expect("Failed to read line");
            grid[i][1..=GRID_SIZE].copy_from_slice(&line.as_bytes()[0..GRID_SIZE]);
        }

        // Convert characters '0' and '1' to u8 values 0 and 1
        for i in 0..GRID_SIZE {
            for j in 1..=GRID_SIZE {
                grid[i][j] -= b'0'; // Convert ASCII to numeric value
            }
        }

        let mut count = 0;

        // Count islands
        for i in 0..GRID_SIZE {
            for j in 1..=GRID_SIZE {
                if grid[i][j] == 1 {
                    count += 1;
                    delete_island(&mut grid, i as isize, j as isize);
                }
            }
        }

        // Output the count of islands
        println!("{}", count);
        
        // Read the next line to handle the newline character
        let _ = reader.lines().next(); // Consume the newline
    }
}
