use std::collections::VecDeque;

const FAILURE_INDICATOR: i32 = -1;

fn min_moves_to_goal(grid: Vec<Vec<char>>, max_moves_x: usize, max_moves_y: usize) -> i32 {
    let height = grid.len();
    let width = grid[0].len();

    // Preconditions check
    if height == 0 || width == 0 || height != grid.iter().map(|row| row.len()).next().unwrap() {
        return FAILURE_INDICATOR;
    }

    let mut start = None;
    let mut goal = None;

    // Locate start and goal positions
    for i in 0..height {
        for j in 0..width {
            match grid[i][j] {
                'S' => {
                    if start.is_none() {
                        start = Some((i, j));
                    } else {
                        return FAILURE_INDICATOR; // More than one start
                    }
                }
                'G' => {
                    if goal.is_none() {
                        goal = Some((i, j));
                    } else {
                        return FAILURE_INDICATOR; // More than one goal
                    }
                }
                _ => {}
            }
        }
    }

    let (start_x, start_y) = start.unwrap();
    let (goal_x, goal_y) = goal.unwrap();

    // BFS setup
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; width]; height];
    queue.push_back((start_x, start_y, 0)); // (x, y, moves)
    visited[start_x][start_y] = true;

    while let Some((x, y, moves)) = queue.pop_front() {
        // Check if goal is reached
        if (x, y) == (goal_x, goal_y) {
            return moves;
        }

        // Explore neighbors
        for dx in 1..=max_moves_x {
            for dy in 1..=max_moves_y {
                let directions = [(dx, 0), (0, dy), (dx, 0), (0, dy)];
                for (dx, dy) in &directions {
                    let new_x = (x as isize + *dx as isize) as usize;
                    let new_y = (y as isize + *dy as isize) as usize;

                    if new_x < height && new_y < width && !visited[new_x][new_y] && grid[new_x][new_y] != 'X' {
                        visited[new_x][new_y] = true;
                        queue.push_back((new_x, new_y, moves + 1));
                    }
                }
            }
        }

        // Check for move threshold
        if moves >= 1000 { // Assuming 1000 is the predefined threshold
            return FAILURE_INDICATOR;
        }
    }

    FAILURE_INDICATOR // Goal not reachable
}

fn main() {
    // Example usage
    let grid = vec![
        vec!['.', '.', 'X', 'G'],
        vec!['.', 'X', '.', '.'],
        vec!['S', '.', 'X', '.'],
    ];
    let result = min_moves_to_goal(grid, 1, 1);
    println!("{}", result);
}
