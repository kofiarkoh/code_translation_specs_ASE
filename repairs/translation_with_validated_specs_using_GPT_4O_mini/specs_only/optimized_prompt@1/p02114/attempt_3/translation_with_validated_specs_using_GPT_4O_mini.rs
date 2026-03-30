fn count_connected_components(n: usize, grid: Vec<Vec<char>>) -> usize {
    // Create a mutable grid to mark visited 'x' characters
    let mut visited = vec![vec![false; n]; n];
    let mut component_count = 0;

    // Directions for moving up, down, left, and right
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    // Helper function for depth-first search (DFS)
    fn dfs(x: isize, y: isize, n: usize, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, directions: &[(isize, isize)]) {
        // Check if the current position is out of bounds or already visited or not 'x'
        if x < 0 || x >= n as isize || y < 0 || y >= n as isize || visited[x as usize][y as usize] || grid[x as usize][y as usize] != 'x' {
            return;
        }
        // Mark the current position as visited
        visited[x as usize][y as usize] = true;

        // Explore all four directions
        for &(dx, dy) in directions {
            dfs(x + dx, y + dy, n, grid, visited, directions);
        }
    }

    // Iterate through each cell in the grid
    for i in 0..n {
        for j in 0..n {
            // If we find an unvisited 'x', we have found a new component
            if grid[i][j] == 'x' && !visited[i][j] {
                component_count += 1; // Increment the component count
                dfs(i as isize, j as isize, n, &grid, &mut visited, &directions); // Perform DFS to mark the entire component
            }
        }
    }

    // Return the number of components divided by three, using integer division
    component_count / 3
}

// Main function to demonstrate usage (optional)
fn main() {
    let n = 5;
    let grid = vec![
        vec!['o', 'x', 'o', 'o', 'o'],
        vec!['x', 'x', 'o', 'x', 'x'],
        vec!['o', 'o', 'x', 'o', 'o'],
        vec!['o', 'x', 'x', 'x', 'x'],
        vec!['o', 'o', 'o', 'x', 'o'],
    ];
    let result = count_connected_components(n, grid);
    println!("{}", result); // Print only the result as expected
}
