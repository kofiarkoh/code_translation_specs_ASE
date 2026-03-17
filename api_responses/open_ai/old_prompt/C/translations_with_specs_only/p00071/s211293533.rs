// Define a constant for the grid size
const GRID_SIZE: usize = 10;

// Function to simulate an explosion effect on a 2D grid
fn explode(field: &mut [[char; GRID_SIZE]; GRID_SIZE], px: usize, py: usize) {
    // Check if the coordinates are within the valid range
    if px < 1 || px > 8 || py < 1 || py > 8 {
        // Undefined behavior as per specification; no handling is done
        return;
    }

    // Convert to zero-based indexing for Rust
    let x = px - 1;
    let y = py - 1;

    // Check if the current cell is '1' (not exploded)
    if field[y][x] == '1' {
        // Mark the cell as exploded
        field[y][x] = '0';

        // Print the current state of the grid after the explosion
        print_grid(field);

        // Recursively trigger explosions in adjacent cells
        // Up
        if y > 0 {
            explode(field, px, py - 1);
        }
        // Down
        if y < GRID_SIZE - 1 {
            explode(field, px, py + 1);
        }
        // Left
        if x > 0 {
            explode(field, px - 1, py);
        }
        // Right
        if x < GRID_SIZE - 1 {
            explode(field, px + 1, py);
        }
    }
}

// Function to print the grid to the console
fn print_grid(field: &[[char; GRID_SIZE]; GRID_SIZE]) {
    for row in field.iter() {
        for &cell in row.iter() {
            print!("{}", cell);
        }
        println!(); // New line after each row
    }
}

// Example usage
fn main() {
    // Initialize a 10x10 grid with '1's
    let mut field: [[char; GRID_SIZE]; GRID_SIZE] = [['1'; GRID_SIZE]; GRID_SIZE];

    // Trigger an explosion at a valid position
    explode(&mut field, 5, 5);
}
