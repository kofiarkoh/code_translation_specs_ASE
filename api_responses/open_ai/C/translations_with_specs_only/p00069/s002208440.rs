fn main() {
    // Example input values
    let columns = 5; // Number of columns
    let target_position = 2; // Target position (0-indexed)
    let rows = 3; // Number of rows
    let grid = vec![
        vec!['0', '1', '0', '0', '1'],
        vec!['0', '0', '1', '0', '0'],
        vec!['1', '0', '0', '1', '0'],
    ];

    // Check preconditions
    if columns <= 0 || target_position < 0 || rows <= 0 {
        return; // Graceful termination if invalid input
    }

    // Check if the initial configuration allows reaching the target position
    if can_reach_target(&grid, target_position) {
        println!("0"); // Output "0" if the target can be reached
    } else {
        // Try to find the first valid move
        if let Some((row, col)) = find_first_valid_move(&grid, target_position) {
            println!("{} {}", row, col); // Output coordinates of the first valid move
        } else {
            println!("1"); // Output "1" if no valid moves can lead to the target position
        }
    }
}

// Function to check if the target position can be reached
fn can_reach_target(grid: &Vec<Vec<char>>, target_position: usize) -> bool {
    // Check if the target position is within bounds
    if target_position >= grid[0].len() {
        return false;
    }
    // Check the first row for a direct path to the target position
    grid[0][target_position] == '0'
}

// Function to find the first valid move that allows reaching the target position
fn find_first_valid_move(grid: &Vec<Vec<char>>, target_position: usize) -> Option<(usize, usize)> {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            // Check if the current position is a valid move
            if grid[row][col] == '1' {
                // Check if moving from this position can reach the target
                if can_reach_from_position(grid, row, col, target_position) {
                    return Some((row, col)); // Return the coordinates of the valid move
                }
            }
        }
    }
    None // Return None if no valid moves are found
}

// Function to check if a move from a given position can reach the target position
fn can_reach_from_position(grid: &Vec<Vec<char>>, start_row: usize, start_col: usize, target_position: usize) -> bool {
    // Simulate the move from the starting position to the target position
    let mut current_col = start_col;
    for row in start_row..grid.len() {
        if current_col > 0 && grid[row][current_col - 1] == '1' {
            current_col -= 1; // Move left
        } else if current_col < grid[row].len() - 1 && grid[row][current_col + 1] == '1' {
            current_col += 1; // Move right
        }
        // Check if we reached the target position
        if row == grid.len() - 1 && current_col == target_position {
            return true;
        }
    }
    false // Return false if the target position is not reachable
}
