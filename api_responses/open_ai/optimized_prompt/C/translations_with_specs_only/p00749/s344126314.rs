use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let dimensions: Vec<&str> = line.split_whitespace().collect();
        
        // Read width and height
        let width: usize = dimensions[0].parse().unwrap();
        let height: usize = dimensions[1].parse().unwrap();

        // Check precondition for width and height
        if width == 0 || height == 0 {
            break; // Terminate if width or height is zero or negative
        }

        // Read the grid
        let mut grid: Vec<Vec<char>> = Vec::new();
        for _ in 0..height {
            let grid_line = reader.lines().next().unwrap().unwrap();
            grid.push(grid_line.chars().collect());
        }

        // Analyze the grid for stability
        let stability = analyze_grid(&grid, width, height);
        println!("{}", stability);
    }
}

// Function to analyze the grid and determine if it is STABLE or UNSTABLE
fn analyze_grid(grid: &[Vec<char>], width: usize, height: usize) -> &'static str {
    // Check connectivity and geometric properties of filled cells
    let mut visited = vec![vec![false; width]; height];
    
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == '#' && !visited[i][j] {
                // Start a new component
                if !dfs(grid, &mut visited, i, j, width, height) {
                    return "UNSTABLE"; // If any component is unstable
                }
            }
        }
    }
    "STABLE" // If all components are stable
}

// Depth-first search to explore filled cells
fn dfs(grid: &[Vec<char>], visited: &mut Vec<Vec<bool>>, x: usize, y: usize, width: usize, height: usize) -> bool {
    if x >= height || y >= width || visited[x][y] || grid[x][y] != '#' {
        return true; // Out of bounds or already visited or not filled
    }

    visited[x][y] = true;

    // Explore neighbors (up, down, left, right)
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for (dx, dy) in directions.iter() {
        let new_x = (x as isize + dx) as usize;
        let new_y = (y as isize + dy) as usize;
        if new_x < height && new_y < width {
            if grid[new_x][new_y] == '#' && !visited[new_x][new_y] {
                // If we find a filled neighbor, continue DFS
                if !dfs(grid, visited, new_x, new_y, width, height) {
                    return false; // If any neighbor leads to instability
                }
            }
        }
    }
    true // All neighbors are stable
}
