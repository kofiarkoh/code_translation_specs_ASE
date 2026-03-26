use std::collections::VecDeque;

fn min_moves(h: usize, w: usize, k: usize, s: (usize, usize), g: (usize, usize), grid: Vec<&str>) -> i32 {
    // Directions for moving in the grid: up, down, left, right
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    
    // Distance array initialized to -1 (unreachable)
    let mut d = vec![vec![-1; w]; h];
    
    // BFS queue
    let mut queue = VecDeque::new();
    
    // Starting position
    let (start_x, start_y) = s;
    d[start_x][start_y] = 0; // Starting point has 0 moves
    queue.push_back((start_x, start_y));
    
    while let Some((x, y)) = queue.pop_front() {
        // Current distance to reach (x, y)
        let current_distance = d[x][y];
        
        // Explore all directions
        for (dx, dy) in &directions {
            // Check all possible steps in the current direction
            for step in 1..=k {
                let new_x = (x as isize + dx * step as isize) as usize;
                let new_y = (y as isize + dy * step as isize) as usize;
                
                // Check if the new position is within bounds
                if new_x < h && new_y < w {
                    // Check if the new position is an obstacle
                    if grid[new_x].chars().nth(new_y).unwrap() == '.' {
                        // If this position has not been reached yet
                        if d[new_x][new_y] == -1 {
                            d[new_x][new_y] = current_distance + 1; // Update distance
                            queue.push_back((new_x, new_y)); // Add to queue for further exploration
                        }
                    } else {
                        // If we hit an obstacle, stop exploring further in this direction
                        break;
                    }
                } else {
                    // Out of bounds, stop exploring further in this direction
                    break;
                }
            }
        }
    }
    
    // Return the result for the goal position
    let (goal_x, goal_y) = g;
    d[goal_x][goal_y]
}

fn main() {
    // Example usage
    let h = 5;
    let w = 5;
    let k = 2;
    let s = (0, 0);
    let g = (4, 4);
    let grid = vec![
        ".....",
        ".....",
        ".....",
        ".....",
        "....."
    ];
    
    let result = min_moves(h, w, k, s, g, grid);
    println!("{}", result); // Output the result
}
