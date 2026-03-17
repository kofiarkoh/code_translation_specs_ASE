use std::collections::VecDeque;

fn max_distance_from_char(grid: Vec<Vec<char>>, target: char) -> i32 {
    // Get the dimensions of the grid
    let height = grid.len();
    if height == 0 {
        return 0; // Handle empty grid case
    }
    let width = grid[0].len();
    
    // Initialize a distance grid with a predefined constant (e.g., -1)
    let mut distance = vec![vec![-1; width]; height];
    let mut queue = VecDeque::new();

    // Set distances for target character cells and enqueue them
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == target {
                distance[i][j] = 0; // Distance for target character is 0
                queue.push_back((i, j)); // Enqueue the position
            }
        }
    }

    // Directions for moving in the grid (up, down, left, right)
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut max_dist = 0;

    // Perform BFS to calculate distances
    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in &directions {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;

            // Check bounds and if the cell has not been visited
            if new_x < height && new_y < width && distance[new_x][new_y] == -1 {
                distance[new_x][new_y] = distance[x][y] + 1; // Update distance
                max_dist = max_dist.max(distance[new_x][new_y]); // Update max distance
                queue.push_back((new_x, new_y)); // Enqueue the new position
            }
        }
    }

    max_dist // Return the maximum distance found
}
