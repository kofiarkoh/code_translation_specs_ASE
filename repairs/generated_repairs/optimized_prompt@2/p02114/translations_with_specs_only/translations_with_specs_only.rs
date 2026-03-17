fn count_connected_components(grid_size: usize, grid: Vec<Vec<char>>, target_char: char) -> usize {
    // Check for invalid grid size; behavior is undefined, so we return 0.
    if grid_size == 0 {
        return 0;
    }

    // Directions for moving in the grid (up, down, left, right)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut visited = vec![vec![false; grid_size]; grid_size];
    let mut component_count = 0;

    // Helper function for depth-first search (DFS)
    fn dfs(x: usize, y: usize, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, directions: &[(isize, isize)], target_char: char) {
        // Mark the current cell as visited
        visited[x][y] = true;

        // Explore all four possible directions
        for &(dx, dy) in directions {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;

            // Check bounds and if the cell is the target character and not visited
            if new_x < grid.len() && new_y < grid[0].len() && 
               !visited[new_x][new_y] && grid[new_x][new_y] == target_char {
                dfs(new_x, new_y, grid, visited, directions, target_char);
            }
        }
    }

    // Iterate through each cell in the grid
    for i in 0..grid_size {
        for j in 0..grid_size {
            // If the cell is the target character and not visited, start a new DFS
            if grid[i][j] == target_char && !visited[i][j] {
                dfs(i, j, &grid, &mut visited, &directions, target_char);
                component_count += 1; // Increment the count of connected components
            }
        }
    }

    // Return the count of distinct connected components of the specified character
    component_count
}

// Main function to run the example
fn main() {
    let grid = vec![
        vec!['o', 'x', 'o', 'o', 'o'],
        vec!['x', 'x', 'o', 'x', 'x'],
        vec!['o', 'o', 'x', 'o', 'o'],
        vec!['o', 'x', 'x', 'x', 'x'],
        vec!['o', 'o', 'o', 'x', 'o'],
    ];
    let result = count_connected_components(5, grid, 'o');
    println!("{}", result); // Expected output: 1
}
