use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    for grid in reader.split(b'\n').collect::<Vec<_>>().chunks(12) {
        if grid.len() < 12 {
            break; // Ensure we only process complete grids
        }
        
        let mut grid_2d = [[0u8; 12]; 12];
        for (i, row) in grid.iter().enumerate() {
            if let Ok(line) = std::str::from_utf8(row) {
                for (j, c) in line.chars().enumerate() {
                    grid_2d[i][j] = c.to_digit(10).unwrap() as u8; // Convert '0'/'1' to 0/1
                }
            }
        }
        
        let island_count = count_islands(&grid_2d);
        println!("{}", island_count);
    }
}

fn count_islands(grid: &[[u8; 12]; 12]) -> usize {
    let mut visited = [[false; 12]; 12];
    let mut count = 0;

    for i in 0..12 {
        for j in 0..12 {
            if grid[i][j] == 1 && !visited[i][j] {
                count += 1;
                mark_island(&grid, &mut visited, i, j);
            }
        }
    }
    
    count
}

fn mark_island(grid: &[[u8; 12]; 12], visited: &mut [[bool; 12]; 12], i: usize, j: usize) {
    if i < 0 || i >= 12 || j < 0 || j >= 12 || visited[i][j] || grid[i][j] == 0 {
        return;
    }
    
    visited[i][j] = true;

    // Check all 4 possible directions (up, down, left, right)
    if i > 0 {
        mark_island(grid, visited, i - 1, j); // Up
    }
    if i < 11 {
        mark_island(grid, visited, i + 1, j); // Down
    }
    if j > 0 {
        mark_island(grid, visited, i, j - 1); // Left
    }
    if j < 11 {
        mark_island(grid, visited, i, j + 1); // Right
    }
}
```

### Explanation of Choices:
1. **Grid Representation**: The grid is represented as a 2D array of `u8`, where '1' is stored as `1` and '0' as `0`. This matches the specification's requirement for valid input.
2. **Input Handling**: The program reads from standard input until EOF, processing each complete grid of 12x12 characters.
3. **Island Counting Logic**: The `count_islands` function iterates through the grid, and when it finds an unvisited land cell, it increments the island count and calls `mark_island` to mark all connected land cells as visited.
4. **Marking Islands**: The `mark_island` function uses recursion to explore all four directions from a land cell, marking them as visited to prevent recounting.
5. **Error Handling**: The program does not handle malformed input as specified, and it breaks out of the loop if a complete grid is not found.
6. **Performance**: The algorithm operates in linear time relative to the number of cells, adhering to the performance expectations outlined in the specification.
