use std::collections::HashSet;

fn count_connected_components(grid_size: usize, grid: Vec<Vec<char>>, target_char: char) -> usize {
    // Check for valid grid size
    if grid_size == 0 {
        return 0; // Undefined behavior for non-positive size, returning 0 as a safe fallback
    }

    let mut visited = HashSet::new();
    let mut component_count = 0;

    // Directions for moving in the grid (up, down, left, right)
    let directions = vec![
        (-1, 0), // up
        (1, 0),  // down
        (0, -1), // left
        (0, 1),  // right
    ];

    // Helper function for depth-first search
    fn dfs(x: isize, y: isize, grid: &Vec<Vec<char>>, visited: &mut HashSet<(isize, isize)>, target_char: char, directions: &Vec<(isize, isize)>) {
        // Check bounds
        if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize {
            return;
        }
        // Check if the cell is the target character and not visited
        if visited.contains(&(x, y)) || grid[x as usize][y as usize] != target_char {
            return;
        }

        // Mark the cell as visited
        visited.insert((x, y));

        // Explore all four directions
        for (dx, dy) in directions.iter() {
            dfs(x + dx, y + dy, grid, visited, target_char, directions);
        }
    }

    // Iterate through each cell in the grid
    for i in 0..grid_size {
        for j in 0..grid_size {
            // If the cell is the target character and not visited, start a new DFS
            if grid[i][j] == target_char && !visited.contains(&(i as isize, j as isize)) {
                dfs(i as isize, j as isize, &grid, &mut visited, target_char, &directions);
                component_count += 1; // Increment the component count
            }
        }
    }

    component_count // Return the count of distinct connected components
}

// Main function to demonstrate usage
fn main() {
    let grid_size = 5;
    let grid = vec![
        vec!['o', 'x', 'o', 'o', 'o'],
        vec!['x', 'x', 'o', 'x', 'x'],
        vec!['o', 'o', 'x', 'o', 'o'],
        vec!['o', 'x', 'x', 'x', 'x'],
        vec!['o', 'o', 'x', 'o', 'o'],
    ];
    let target_char = 'o';
    let result = count_connected_components(grid_size, grid, target_char);
    println!("{}", result); // Expected output: 1
}
