fn count_connected_components(n: usize, grid: &mut [[char; 300]; 300]) -> i32 {
    // Check for valid input size
    if n == 0 || n > 300 {
        return 0; // Undefined behavior for invalid n, returning 0 as a safe fallback
    }

    let mut count = 0;

    // Directions for moving in the grid (up, down, left, right)
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    // Helper function for depth-first search (DFS)
    fn dfs(x: isize, y: isize, n: usize, grid: &mut [[char; 300]; 300]) {
        if x < 0 || x >= n as isize || y < 0 || y >= n as isize || grid[x as usize][y as usize] != 'o' {
            return; // Out of bounds or not an 'o'
        }
        grid[x as usize][y as usize] = 'x'; // Mark as visited
        for &(dx, dy) in &directions {
            dfs(x + dx, y + dy, n, grid); // Explore neighbors
        }
    }

    // Iterate through the grid
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 'o' {
                count += 1; // Found a new component
                dfs(i as isize, j as isize, n, grid); // Mark all connected 'o's
            }
        }
    }

    count / 3 // Return the count divided by three
}

fn main() {
    let n = 5; // Example size of the grid
    let mut grid: [[char; 300]; 300] = [['x'; 300]; 300]; // Initialize grid with 'x'
    
    // Example grid initialization (this would be replaced with actual input)
    grid[0][0] = 'o';
    grid[0][1] = 'o';
    grid[1][0] = 'o';
    grid[2][2] = 'o';
    
    let result = count_connected_components(n, &mut grid);
    println!("{}", result); // Output the result
}
