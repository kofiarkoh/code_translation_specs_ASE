// Define the size of the grid
const GRID_SIZE: usize = 10;

// Function to process the explosion effect on the grid
fn explode_grid(grid: &mut [[char; GRID_SIZE]; GRID_SIZE], start_x: usize, start_y: usize, target: char, replacement: char) {
    // Check if starting coordinates are within bounds
    if start_x >= GRID_SIZE || start_y >= GRID_SIZE {
        return; // Out of bounds, do nothing
    }

    // Check if the starting cell is the target value
    if grid[start_x][start_y] != target {
        return; // Starting cell does not trigger explosion
    }

    // Use a stack for DFS to propagate the explosion effect
    let mut stack = vec![(start_x, start_y)];

    while let Some((x, y)) = stack.pop() {
        // Change the current cell to the replacement value
        grid[x][y] = replacement;

        // Check adjacent cells (up, down, left, right)
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions.iter() {
            let new_x = x.wrapping_add(*dx);
            let new_y = y.wrapping_add(*dy);

            // Check if the new coordinates are within bounds and match the target value
            if new_x < GRID_SIZE && new_y < GRID_SIZE && grid[new_x][new_y] == target {
                stack.push((new_x, new_y)); // Add to stack for further processing
            }
        }
    }
}

// Function to print the grid
fn print_grid(grid: &[[char; GRID_SIZE]; GRID_SIZE]) {
    for row in grid.iter() {
        for &cell in row.iter() {
            print!("{}", cell);
        }
        println!(); // New line after each row
    }
}

// Main function to handle input and output
fn main() {
    // Example grid initialization (this should be replaced with actual input handling)
    let mut grid: [[char; GRID_SIZE]; GRID_SIZE] = [
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', 'X', 'X', '.', '.', '.', '.', '.', '.', '.'],
        ['.', 'X', 'X', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ];

    // Number of test cases (this should be replaced with actual input handling)
    let test_cases = 1;

    // Process each test case
    for _ in 0..test_cases {
        // Example starting coordinates and values (this should be replaced with actual input handling)
        let start_x = 1;
        let start_y = 1;
        let target = 'X';
        let replacement = '*';

        // Call the explosion function
        explode_grid(&mut grid, start_x, start_y, target, replacement);

        // Print the modified grid
        print_grid(&grid);
    }
}
