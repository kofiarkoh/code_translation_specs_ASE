use std::collections::VecDeque;

fn min_moves(h: usize, w: usize, k: usize, start: (usize, usize), goal: (usize, usize), grid: Vec<Vec<char>>) -> i32 {
    // Directions for moving in the grid: up, down, left, right
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    
    // Distance array initialized to a large value (infinity)
    let mut d = vec![vec![i32::MAX; w]; h];
    
    // BFS queue
    let mut queue = VecDeque::new();
    
    // Starting position
    let (start_x, start_y) = start;
    d[start_x][start_y] = 0;
    queue.push_back((start_x, start_y));
    
    // BFS traversal
    while let Some((x, y)) = queue.pop_front() {
        // Check all four directions
        for (dx, dy) in &directions {
            // Explore up to k steps in the current direction
            for step in 1..=k {
                let new_x = (x as isize + dx * step as isize) as usize;
                let new_y = (y as isize + dy * step as isize) as usize;
                
                // Check if the new position is within bounds
                if new_x < h && new_y < w {
                    // Check if the position is an obstacle
                    if grid[new_x][new_y] == '.' {
                        // If we found a shorter path to this position
                        if d[new_x][new_y] > d[x][y] + 1 {
                            d[new_x][new_y] = d[x][y] + 1;
                            queue.push_back((new_x, new_y));
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
    
    // Get the result for the goal position
    let (goal_x, goal_y) = goal;
    let result = d[goal_x][goal_y];
    
    // If the goal is unreachable, return -1
    if result == i32::MAX {
        -1
    } else {
        result
    }
}

fn main() {
    // Example usage
    let h = 5;
    let w = 5;
    let k = 2;
    let start = (0, 0);
    let goal = (4, 4);
    let grid = vec![
        vec!['.', '.', '.', '.', '.'],
        vec!['.', '#', '#', '#', '.'],
        vec!['.', '.', '.', '#', '.'],
        vec!['.', '#', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.'],
    ];
    
    let result = min_moves(h, w, k, start, goal, grid);
    println!("{}", result); // Output the result
}
