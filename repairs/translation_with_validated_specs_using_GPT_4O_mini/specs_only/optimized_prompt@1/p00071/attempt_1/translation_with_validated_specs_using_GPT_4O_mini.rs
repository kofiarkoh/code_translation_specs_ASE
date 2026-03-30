fn main() {
    let datasets = 1; // Example: number of datasets to process
    let mut grids = vec![
        [
            ['1', '1', '0', '0', '0', '0', '0', '0'],
            ['1', '1', '0', '1', '1', '0', '0', '0'],
            ['0', '0', '0', '0', '1', '1', '1', '0'],
            ['0', '1', '1', '0', '0', '0', '0', '0'],
            ['0', '0', '0', '0', '0', '0', '0', '0'],
            ['0', '0', '0', '1', '1', '1', '0', '0'],
            ['0', '0', '0', '0', '0', '0', '0', '0'],
            ['0', '0', '0', '0', '0', '0', '0', '0'],
        ],
    ];

    // Example starting coordinates for the explosion
    let start_x = 2; // 1-based index
    let start_y = 2; // 1-based index

    for i in 0..datasets {
        let grid = &mut grids[i];
        // Convert 1-based to 0-based index
        let x = start_x - 1;
        let y = start_y - 1;

        // Check if starting coordinates are within bounds
        if x >= 0 && x < 8 && y >= 0 && y < 8 {
            explode(grid, x, y);
        }

        // Print the modified grid
        for row in grid.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

fn explode(grid: &mut [[char; 8]; 8], x: usize, y: usize) {
    // Base case: if out of bounds or not '1', return
    if x >= 8 || y >= 8 || grid[x][y] != '1' {
        return;
    }

    // Set the current cell to '0'
    grid[x][y] = '0';

    // Recursively explode in all four directions
    explode(grid, x.wrapping_sub(1), y); // Up
    explode(grid, x + 1, y); // Down
    explode(grid, x, y.wrapping_sub(1)); // Left
    explode(grid, x, y + 1); // Right
}
