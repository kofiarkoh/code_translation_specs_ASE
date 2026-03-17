fn count_adjacent_unmarked_cells(w: usize, h: usize, grid: &mut Vec<Vec<i32>>) -> i32 {
    // Check for valid dimensions
    if w == 0 || h == 0 {
        // Undefined behavior for empty grid, returning 0 as a safe fallback
        return 0;
    }

    // Directions for adjacent cells (up, down, left, right)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut count = 0;

    // Marked cells will be represented by a specific value, e.g., -1
    for i in 0..h {
        for j in 0..w {
            // If the current cell is marked (e.g., -1), check its neighbors
            if grid[i][j] == -1 {
                for &(dx, dy) in &directions {
                    let ni = i as isize + dy; // new row index
                    let nj = j as isize + dx; // new column index

                    // Check bounds
                    if ni >= 0 && ni < h as isize && nj >= 0 && nj < w as isize {
                        let ni = ni as usize;
                        let nj = nj as usize;

                        // If the neighbor is unmarked (not -1), mark it and increment count
                        if grid[ni][nj] != -1 {
                            grid[ni][nj] = -1; // Mark the cell
                            count += 1; // Increment the count of adjacent unmarked cells
                        }
                    }
                }
            }
        }
    }

    count // Return the count of unmarked cells adjacent to marked cells
}
