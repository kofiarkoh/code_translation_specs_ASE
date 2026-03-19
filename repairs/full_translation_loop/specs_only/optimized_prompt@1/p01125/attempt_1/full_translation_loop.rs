use std::collections::HashSet;

fn process_commands(num_marked: usize, marked_positions: Vec<(usize, usize)>, num_commands: usize, commands: Vec<(char, usize)>) -> String {
    // Check preconditions for positive integers
    if num_marked == 0 || num_commands == 0 {
        return String::new(); // Terminate without output
    }

    // Initialize the grid and the current position
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut current_position = (0, 0); // Starting at the top-left corner (0, 0)

    // Mark the specified positions as visited
    for pos in marked_positions {
        visited.insert(pos);
    }

    // Process each command
    for (direction, steps) in commands {
        // Move according to the direction
        for _ in 0..steps {
            match direction {
                'U' => {
                    if current_position.1 > 0 {
                        current_position.1 -= 1; // Move up
                    }
                }
                'D' => {
                    if current_position.1 < 24 {
                        current_position.1 += 1; // Move down
                    }
                }
                'L' => {
                    if current_position.0 > 0 {
                        current_position.0 -= 1; // Move left
                    }
                }
                'R' => {
                    if current_position.0 < 24 {
                        current_position.0 += 1; // Move right
                    }
                }
                _ => {} // Ignore invalid directions
            }

            // Check if the new position is a marked position
            visited.remove(&current_position);
        }
    }

    // Determine if all marked positions have been visited
    if visited.is_empty() {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}
