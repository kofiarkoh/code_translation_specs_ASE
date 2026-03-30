fn count_adjacent_to_twos(w: usize, h: usize, grid: &mut Vec<Vec<i32>>) -> i32 {
    // Define movement vectors for the six directions (up, down, left, right, and two diagonals)
    let directions: [(isize, isize); 6] = [
        (-1, 0), // up
        (1, 0),  // down
        (0, -1), // left
        (0, 1),  // right
        (-1, -1), // up-left
        (-1, 1),  // up-right
        (1, -1),  // down-left
        (1, 1),   // down-right
    ];

    // Create a temporary grid to mark adjacent cells
    let mut marked = vec![vec![false; 2 * w + 4]; h];

    // Mark cells in the grid based on the traversal logic
    for y in 0..h {
        for x in 0..(2 * w + 4) {
            if grid[y][x] == 2 {
                // Mark adjacent cells as "2" if they are within bounds and not already marked
                for (dy, dx) in &directions {
                    let new_y = y as isize + dy;
                    let new_x = x as isize + dx;
                    if new_y >= 0 && new_y < h as isize && new_x >= 0 && new_x < (2 * w + 4) as isize {
                        let new_y = new_y as usize;
                        let new_x = new_x as usize;
                        marked[new_y][new_x] = true; // Mark the cell as adjacent
                    }
                }
            }
        }
    }

    // Count cells that are adjacent to at least one marked cell
    let mut count = 0;
    for y in 0..h {
        for x in 0..(2 * w + 4) {
            if marked[y][x] && grid[y][x] != 2 {
                count += 1; // Count only if the adjacent cell is not already marked
            }
        }
    }

    count
}

fn main() {
    // Example usage
    let w = 4; // Width of the grid
    let h = 3; // Height of the grid
    let mut grid = vec![
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    ];
    
    let result = count_adjacent_to_twos(w, h, &mut grid);
    println!("Count of adjacent cells: {}", result);
}
