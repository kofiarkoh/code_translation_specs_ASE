use std::collections::VecDeque;

const BLOCKED: char = '#'; // Represents a blocked cell
const TRAVERSABLE: char = '.'; // Represents a traversable cell
const SENTINEL: i64 = -1; // Sentinel value for no valid flow

fn min_cost_flow(rows: usize, cols: usize, grid: Vec<Vec<char>>) -> i64 {
    // Check for valid grid dimensions
    if rows == 0 || cols == 0 || grid.len() != rows || grid.iter().any(|r| r.len() != cols) {
        return SENTINEL; // Invalid grid, return sentinel
    }

    // Initialize distances
    let mut distances = vec![vec![i64::MAX; cols]; rows];

    // BFS or similar algorithm to find the minimum cost flow
    let mut queue = VecDeque::new();
    queue.push_back((0, 0)); // Start from the top-left corner
    distances[0][0] = 0; // Starting point

    while let Some((x, y)) = queue.pop_front() {
        // Check all four possible directions
        for (dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            // Check bounds and if the cell is traversable
            if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid[nx][ny] == TRAVERSABLE {
                    // Update distance if a shorter path is found
                    if distances[nx][ny] > distances[x][y] + 1 {
                        distances[nx][ny] = distances[x][y] + 1;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }

    // Check if we reached the bottom-right corner
    if distances[rows - 1][cols - 1] == i64::MAX {
        return SENTINEL; // No valid flow found
    }

    // Calculate the minimum cost based on distances
    let min_cost = distances[rows - 1][cols - 1];

    min_cost
}

fn main() {
    // Example usage
    let rows = 3;
    let cols = 3;
    let grid = vec![
        vec![TRAVERSABLE, TRAVERSABLE, BLOCKED],
        vec![BLOCKED, TRAVERSABLE, TRAVERSABLE],
        vec![TRAVERSABLE, BLOCKED, TRAVERSABLE],
    ];

    let result = min_cost_flow(rows, cols, grid);
    println!("Minimum cost of flow: {}", result);
}
