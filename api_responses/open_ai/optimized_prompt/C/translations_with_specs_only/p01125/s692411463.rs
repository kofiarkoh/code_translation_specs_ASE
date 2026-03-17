// Define a struct to represent the grid and its obstacles
struct Grid {
    obstacles: Vec<(i32, i32)>, // Store obstacle coordinates as pairs of integers
}

impl Grid {
    // Create a new grid with specified obstacles
    fn new(obstacles: Vec<(i32, i32)>) -> Self {
        Grid { obstacles }
    }

    // Process movement commands and update the state of obstacles
    fn process_movements(&mut self, movements: Vec<(char, i32)>) {
        let mut current_position = (0, 0); // Start at the origin (0, 0)

        for (direction, steps) in movements {
            for _ in 0..steps {
                // Update the current position based on the direction
                match direction {
                    'N' => current_position.1 += 1, // Move North
                    'S' => current_position.1 -= 1, // Move South
                    'E' => current_position.0 += 1, // Move East
                    'W' => current_position.0 -= 1, // Move West
                    _ => {} // Invalid direction, do nothing
                }

                // Remove obstacle if the current position matches any obstacle
                self.obstacles.retain(|&obstacle| obstacle != current_position);
            }
        }
    }

    // Check if any obstacles remain
    fn obstacles_remaining(&self) -> bool {
        !self.obstacles.is_empty() // Return true if there are obstacles left
    }
}

// Main function to execute the program logic
fn main() {
    // Example input (this would typically come from user input or a file)
    let num_obstacles = 3; // Number of obstacles
    let obstacle_coords = vec![(1, 1), (2, 2), (3, 3)]; // Coordinates of obstacles
    let num_movements = 2; // Number of movement commands
    let movement_commands = vec![('N', 2), ('E', 3)]; // Movement commands

    // Create a grid with the specified obstacles
    let mut grid = Grid::new(obstacle_coords);

    // Process the movement commands
    grid.process_movements(movement_commands);

    // Output the result based on whether obstacles remain
    if grid.obstacles_remaining() {
        println!("No"); // At least one obstacle remains
    } else {
        println!("Yes"); // No obstacles remain
    }
}
