fn main() {
    // Example input values
    let marked_positions = vec![(1, 1), (2, 2), (3, 3)];
    let movement_commands = vec![('R', 2), ('D', 2)];

    // Call the function to check if all marked positions are visited
    let result = check_visited(&marked_positions, &movement_commands);
    println!("{}", result);
}

fn check_visited(marked_positions: &[(usize, usize)], movement_commands: &[(char, usize)]) -> String {
    // Preconditions: Check if the number of marked positions and movement commands are positive
    if marked_positions.is_empty() || movement_commands.is_empty() {
        return String::new(); // Terminate without output if preconditions are not met
    }

    // Initialize the grid and the starting position
    let mut visited = [[false; 25]; 25]; // 25x25 grid
    let mut current_position = (0, 0); // Starting at (0, 0)

    // Mark the specified positions as visited
    for &(x, y) in marked_positions {
        if x < 25 && y < 25 {
            visited[x][y] = true; // Mark the position as visited
        }
    }

    // Process each movement command
    for &(direction, steps) in movement_commands {
        for _ in 0..steps {
            match direction {
                'U' => {
                    if current_position.0 > 0 {
                        current_position.0 -= 1; // Move up
                    }
                }
                'D' => {
                    if current_position.0 < 24 {
                        current_position.0 += 1; // Move down
                    }
                }
                'L' => {
                    if current_position.1 > 0 {
                        current_position.1 -= 1; // Move left
                    }
                }
                'R' => {
                    if current_position.1 < 24 {
                        current_position.1 += 1; // Move right
                    }
                }
                _ => {} // Ignore invalid directions
            }

            // Mark the current position as visited if it's a marked position
            if visited[current_position.0][current_position.1] {
                visited[current_position.0][current_position.1] = false; // Mark as visited
            }
        }
    }

    // Check if all marked positions have been visited
    for &(x, y) in marked_positions {
        if x < 25 && y < 25 && visited[x][y] {
            return "No".to_string(); // Return "No" if any marked position is unvisited
        }
    }

    "Yes".to_string() // Return "Yes" if all marked positions are visited
}
