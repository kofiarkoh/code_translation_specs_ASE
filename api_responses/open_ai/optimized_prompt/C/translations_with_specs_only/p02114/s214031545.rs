fn count_connected_components(grid_size: usize, grid: Vec<Vec<char>>, target_char: char) -> usize {
    // Ensure the grid size is positive
    if grid_size == 0 || grid.len() != grid_size || grid.iter().any(|row| row.len() != grid_size) {
        return 0; // Invalid grid, return 0 as a safe fallback
    }

    let mut visited = vec![vec![false; grid_size]; grid_size];
    let mut count = 0;

    // Directions for moving in the grid (up, down, left, right)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Helper function for depth-first search
    fn dfs(x: isize, y: isize, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, target_char: char) {
        if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid.len() as isize {
            return; // Out of bounds
        }
        if visited[x as usize][y as usize] || grid[x as usize][y as usize] != target_char {
            return; // Already visited or not the target character
        }

        visited[x as usize][y as usize] = true; // Mark as visited

        // Explore all four directions
        for &(dx, dy) in &directions {
            dfs(x + dx, y + dy, grid, visited, target_char);
        }
    }

    // Iterate through each cell in the grid
    for i in 0..grid_size {
        for j in 0..grid_size {
            if grid[i][j] == target_char && !visited[i][j] {
                count += 1; // Found a new connected component
                dfs(i as isize, j as isize, &grid, &mut visited, target_char); // Explore the component
            }
        }
    }

    count // Return the adjusted count of distinct connected components
}
