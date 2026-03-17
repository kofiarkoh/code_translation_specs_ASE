// Rust implementation of the connected components counting in a 2D character grid.

fn count_connected_components(H: usize, W: usize, grid: &mut [[char; 128]; 128]) -> usize {
    // Validate input dimensions
    if H == 0 && W == 0 {
        return 0; // Termination case as per specification
    }

    // Initialize area_id and area_num arrays
    let mut area_id = [[-1; 128]; 128]; // Using -1 to indicate unvisited cells
    let mut area_num = [0; 128 * 128]; // 1D array to track unique areas
    let mut component_count = 0;

    // Directions for moving up, down, left, right
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Depth-First Search (DFS) function
    fn dfs(
        x: usize,
        y: usize,
        component_id: i32,
        grid: &mut [[char; 128]; 128],
        area_id: &mut [[i32; 128]; 128],
        directions: &[(isize, isize)],
    ) {
        // Mark the current cell with the component ID
        area_id[x][y] = component_id;

        // Explore all four directions
        for &(dx, dy) in directions {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;

            // Check bounds and if the cell is part of the same component
            if new_x < 128 && new_y < 128 && area_id[new_x][new_y] == -1 && grid[new_x][new_y] == grid[x][y] {
                dfs(new_x, new_y, component_id, grid, area_id, directions);
            }
        }
    }

    // Iterate through each cell in the grid
    for i in 0..H {
        for j in 0..W {
            // If the cell is unvisited, perform DFS
            if area_id[i][j] == -1 {
                component_count += 1; // Found a new component
                dfs(i, j, component_count as i32, grid, &mut area_id, &directions);
                area_num[component_count - 1] = 1; // Mark this area as counted
            }
        }
    }

    component_count // Return the count of unique connected components
}

// Example usage (not part of the specification, just for demonstration)
// fn main() {
//     let mut grid = [[' '; 128]; 128];
//     let H = 5;
//     let W = 5;
//     // Fill the grid with some characters
//     // Call count_connected_components(H, W, &mut grid);
// }
