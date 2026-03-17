// Define the size of the grid
const GRID_SIZE: usize = 10;

// Define the specific value that triggers an explosion
const TRIGGER_VALUE: char = 'X';
// Define the value that indicates an exploded cell
const EXPLODED_VALUE: char = '*';

// Function to process the grid and perform the explosion
fn process_grid(grid: &mut [[char; GRID_SIZE]; GRID_SIZE], start_x: usize, start_y: usize) {
    // Check if the starting coordinates are within bounds
    if start_x < 1 || start_x > 8 || start_y < 1 || start_y > 8 {
        return; // Out of bounds, do nothing
    }

    // Adjust for 0-based indexing
    let x = start_x - 1;
    let y = start_y - 1;

    // Check if the starting cell is the trigger value
    if grid[y][x] != TRIGGER_VALUE {
        return; // No explosion if the trigger value is not present
    }

    // Perform the explosion by marking the starting cell
    grid[y][x] = EXPLODED_VALUE;

    // Define a recursive function to propagate the explosion
    fn explode(grid: &mut [[char; GRID_SIZE]; GRID_SIZE], x: usize, y: usize) {
        // Directions for propagation: up, down, left, right
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (dx, dy) in directions.iter() {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            // Check bounds
            if new_x >= 0 && new_x < GRID_SIZE as isize && new_y >= 0 && new_y < GRID_SIZE as isize {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                // If the adjacent cell is the trigger value, explode it
                if grid[new_y][new_x] == TRIGGER_VALUE {
                    grid[new_y][new_x] = EXPLODED_VALUE; // Mark as exploded
                    explode(grid, new_x, new_y); // Recursive call
                }
            }
        }
    }

    // Start the explosion propagation
    explode(grid, x, y);
}

// Function to print the grid
fn print_grid(grid: &[[char; GRID_SIZE]; GRID_SIZE]) {
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

// Main function to handle input and output
fn main() {
    // Example grid initialization (this would typically come from input)
    let mut grid: [[char; GRID_SIZE]; GRID_SIZE] = [
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
    ];

    // Example of setting a trigger value for testing
    grid[4][4] = TRIGGER_VALUE; // Set a trigger value at (5, 5)

    // Example starting coordinates for the explosion
    let start_x = 5; // 1-based index
    let start_y = 5; // 1-based index

    // Process the grid for the explosion
    process_grid(&mut grid, start_x, start_y);

    // Print the modified grid
    print_grid(&grid);
}
