// Import necessary modules
use std::io::{self, BufRead};

// Define the maximum grid size constants
const MAX_HEIGHT: usize = 128;
const MAX_WIDTH: usize = 128;

// Function to count distinct connected areas in a grid
fn count_connected_areas(grid: &Vec<Vec<char>>, height: usize, width: usize) -> usize {
    // Create a visited array to track which cells have been processed
    let mut visited = vec![vec![false; width]; height];
    let mut area_count = 0;

    // Directions for adjacent cells (up, down, left, right)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Depth-first search to explore connected areas
    fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize, height: usize, width: usize, directions: &[(isize, isize); 4]) {
        visited[x][y] = true; // Mark the cell as visited
        for &(dx, dy) in directions {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;
            // Check bounds and if the cell is the same character and not visited
            if new_x < height && new_y < width && !visited[new_x][new_y] && grid[new_x][new_y] == grid[x][y] {
                dfs(grid, visited, new_x, new_y, height, width, directions); // Recursive call
            }
        }
    }

    // Iterate through each cell in the grid
    for i in 0..height {
        for j in 0..width {
            // If the cell is not visited, it's a new area
            if !visited[i][j] {
                area_count += 1; // Increment area count
                dfs(grid, &mut visited, i, j, height, width, &directions); // Explore the area
            }
        }
    }

    area_count // Return the total count of distinct areas
}

// Main function to read input and output the result
fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let dimensions: Vec<usize> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Read height and width
        if dimensions.len() != 2 {
            continue; // Skip if not exactly two dimensions
        }
        let height = dimensions[0];
        let width = dimensions[1];

        // Handle the case where both height and width are zero
        if height == 0 && width == 0 {
            break; // Terminate gracefully
        }

        // Ensure dimensions do not exceed maximum limits
        if height > MAX_HEIGHT || width > MAX_WIDTH {
            continue; // Skip invalid dimensions
        }

        // Read the grid
        let mut grid = Vec::new();
        for _ in 0..height {
            if let Some(line) = lines.next() {
                let line = line.unwrap();
                grid.push(line.chars().collect());
            }
        }

        // Count distinct connected areas and print the result
        let area_count = count_connected_areas(&grid, height, width);
        println!("{}", area_count);
    }
}
