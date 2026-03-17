use std::collections::VecDeque;

const OBSTACLE: char = '#'; // Character representing an obstacle
const PASSABLE: char = '.'; // Character representing a passable cell

fn min_moves_to_target(
    height: usize,
    width: usize,
    max_step: usize,
    start: (usize, usize),
    target: (usize, usize),
    grid: Vec<Vec<char>>,
) -> i32 {
    // Check if the starting and target positions are within bounds
    if !is_within_bounds(start, height, width) || !is_within_bounds(target, height, width) {
        return -1; // Undefined behavior for out-of-bounds
    }

    // Check if the starting or target positions are on obstacles
    if grid[start.0][start.1] == OBSTACLE || grid[target.0][target.1] == OBSTACLE {
        return -1; // Target is unreachable if on an obstacle
    }

    // Distance array to track minimum moves to each cell
    let mut distance = vec![vec![-1; width]; height];
    distance[start.0][start.1] = 0; // Starting position has 0 moves

    // Queue for BFS
    let mut queue = VecDeque::new();
    queue.push_back(start);

    // Directions for movement (up, down, left, right)
    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    while let Some((x, y)) = queue.pop_front() {
        // Explore all possible steps from the current position
        for (dx, dy) in &directions {
            for step in 1..=max_step {
                let new_x = (x as isize + dx * step as isize) as usize;
                let new_y = (y as isize + dy * step as isize) as usize;

                // Check if the new position is within bounds and passable
                if is_within_bounds((new_x, new_y), height, width) && grid[new_x][new_y] == PASSABLE {
                    // If this cell has not been visited yet
                    if distance[new_x][new_y] == -1 {
                        distance[new_x][new_y] = distance[x][y] + 1; // Update distance
                        queue.push_back((new_x, new_y)); // Add to queue

                        // If we reached the target, return the distance
                        if (new_x, new_y) == target {
                            return distance[new_x][new_y];
                        }
                    }
                } else {
                    // Stop exploring further in this direction if we hit an obstacle or out of bounds
                    break;
                }
            }
        }
    }

    // If we finish processing and haven't reached the target, return -1
    -1
}

// Helper function to check if coordinates are within grid bounds
fn is_within_bounds(pos: (usize, usize), height: usize, width: usize) -> bool {
    pos.0 < height && pos.1 < width
}

fn main() {
    // Example usage (this part can be modified or removed as needed)
    let height = 5;
    let width = 5;
    let max_step = 2;
    let start = (0, 0);
    let target = (4, 4);
    let grid = vec![
        vec!['.', '.', '.', '.', '.'],
        vec!['.', '#', '#', '#', '.'],
        vec!['.', '.', '.', '#', '.'],
        vec!['.', '#', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.'],
    ];

    let result = min_moves_to_target(height, width, max_step, start, target, grid);
    println!("{}", result); // Output the result
}
