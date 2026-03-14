fn check_survival(start_x: i32, start_y: i32, num_obstacles: usize, obstacles: &[(i32, i32)]) -> String {
    // Preconditions: Check if starting coordinates are both zero
    if start_x == 0 && start_y == 0 {
        return "NA".to_string(); // Graceful termination if starting position is (0, 0)
    }

    // Check if the number of obstacles exceeds the maximum allowed
    // Assuming a maximum limit for demonstration; this should be defined based on the specification
    const MAX_OBSTACLES: usize = 100; // Example maximum limit
    if num_obstacles > MAX_OBSTACLES {
        return "NA".to_string(); // Return "NA" if the number of obstacles exceeds the limit
    }

    // Invariants: The starting position and obstacle positions remain unchanged
    // Check if the starting position is occupied by an obstacle
    for &(obstacle_x, obstacle_y) in obstacles.iter() {
        if obstacle_x == start_x && obstacle_y == start_y {
            return "NA".to_string(); // Return "NA" if the starting position is occupied by an obstacle
        }
    }

    // If no obstacles are at the starting position, survival is possible
    "OK".to_string()
}

// Example usage
fn main() {
    let start_x = 1;
    let start_y = 1;
    let num_obstacles = 2;
    let obstacles = [(1, 2), (2, 2)];

    let result = check_survival(start_x, start_y, num_obstacles, &obstacles);
    println!("{}", result); // Output the survival status
}
```

### Explanation of Choices:
- **Graceful Termination**: The function checks if the starting coordinates are both zero and returns "NA" as specified.
- **Obstacle Check**: The function iterates through the list of obstacles to determine if the starting position is occupied.
- **Maximum Obstacles**: A constant `MAX_OBSTACLES` is defined to handle the case where the number of obstacles exceeds a reasonable limit, returning "NA" in such cases.
- **Return Type**: The function returns a `String` to match the output requirements of the specification.
