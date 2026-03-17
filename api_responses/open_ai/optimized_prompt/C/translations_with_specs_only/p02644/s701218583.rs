use std::collections::VecDeque;

const UNREACHABLE: i32 = -1;

fn min_moves(height: usize, width: usize, max_step: usize, start: (usize, usize), target: (usize, usize), grid: &Vec<Vec<char>>) -> i32 {
    // Check for valid grid dimensions and positions
    if height == 0 || width == 0 || !is_within_bounds(start, height, width) || !is_within_bounds(target, height, width) {
        return UNREACHABLE; // Undefined behavior for invalid inputs
    }

    // Initialize distance array with UNREACHABLE constant
    let mut distance = vec![vec![UNREACHABLE; width]; height];
    distance[start.0][start.1] = 0; // Starting position has 0 moves

    // Directions for movement (up, down, left, right)
    let directions = vec![
        (1, 0), // down
        (0, 1), // right
        (-1, 0), // up
        (0, -1), // left
    ];

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some((x, y)) = queue.pop_front() {
        // Explore all possible moves within the max step size
        for dx in 1..=max_step {
            for dy in 1..=max_step {
                for (dx_sign, dy_sign) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let new_x = x as isize + dx_sign * dx as isize;
                    let new_y = y as isize + dy_sign * dy as isize;

                    if is_within_bounds((new_x as usize, new_y as usize), height, width) && grid[new_x as usize][new_y as usize] != '#' {
                        // If the new position is reachable and not visited
                        if distance[new_x as usize][new_y as usize] == UNREACHABLE {
                            distance[new_x as usize][new_y as usize] = distance[x][y] + 1;
                            queue.push_back((new_x as usize, new_y as usize));
                        }
                    }
                }
            }
        }
    }

    // Return the distance to the target position or -1 if unreachable
    distance[target.0][target.1]
}

// Helper function to check if a position is within grid bounds
fn is_within_bounds(pos: (usize, usize), height: usize, width: usize) -> bool {
    pos.0 < height && pos.1 < width
}
