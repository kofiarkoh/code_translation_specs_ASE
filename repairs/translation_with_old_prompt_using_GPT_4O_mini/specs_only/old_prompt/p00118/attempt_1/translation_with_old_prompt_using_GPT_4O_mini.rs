fn count_connected_components(H: usize, W: usize, grid: &mut [[char; 128]; 128], area_id: &mut [[i32; 128]; 128], area_num: &mut [char; 128 * 128]) -> i32 {
    // Check for the termination condition
    if H == 0 && W == 0 {
        return 0; // Terminate if both dimensions are zero
    }

    let mut component_count = 0;

    // Directions for moving up, down, left, right
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // Depth-First Search (DFS) function
    fn dfs(x: isize, y: isize, char_to_match: char, area_id: &mut [[i32; 128]; 128], grid: &[[char; 128]; 128], component_id: i32, H: usize, W: usize) {
        // Check bounds
        if x < 0 || x >= H as isize || y < 0 || y >= W as isize {
            return;
        }
        // Check if the current cell matches the character and is not already visited
        if grid[x as usize][y as usize] != char_to_match || area_id[x as usize][y as usize] != -1 {
            return;
        }

        // Mark the current cell with the component ID
        area_id[x as usize][y as usize] = component_id;

        // Explore all four directions
        for &(dx, dy) in &directions {
            dfs(x + dx, y + dy, char_to_match, area_id, grid, component_id, H, W);
        }
    }

    // Initialize area_id with -1 to indicate unvisited cells
    for i in 0..H {
        for j in 0..W {
            area_id[i][j] = -1; // Mark all cells as unvisited
        }
    }

    // Iterate through each cell in the grid
    for i in 0..H {
        for j in 0..W {
            // If the cell is unvisited, perform DFS
            if area_id[i][j] == -1 {
                component_count += 1; // Found a new component
                dfs(i as isize, j as isize, grid[i][j], area_id, grid, component_count, H, W);
                area_num[(component_count - 1) as usize] = grid[i][j]; // Mark the area_num with the character
            }
        }
    }

    component_count // Return the count of distinct connected components
}
