use std::collections::HashSet;

fn count_distinct_pairs(height: usize, width: usize, grid: Vec<Vec<char>>) -> Result<usize, &'static str> {
    // Validate input dimensions
    if height == 0 || width == 0 || height > 512 || width > 512 {
        return Err("Invalid grid dimensions");
    }
    
    // Validate grid structure
    if grid.len() != height || grid.iter().any(|row| row.len() != width) {
        return Err("Grid does not match specified dimensions");
    }

    // Directions for adjacent cells (up, down, left, right)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut group_id = vec![vec![-1; width]; height]; // Group identifiers
    let mut current_group = 0;

    // Function to perform DFS and mark groups
    fn dfs(grid: &Vec<Vec<char>>, group_id: &mut Vec<Vec<i32>>, x: usize, y: usize, current_group: i32, directions: &[(isize, isize)]) {
        let char = grid[x][y];
        let height = grid.len();
        let width = grid[0].len();
        let mut stack = vec![(x, y)];

        while let Some((cx, cy)) = stack.pop() {
            if group_id[cx][cy] != -1 {
                continue; // Already visited
            }
            group_id[cx][cy] = current_group;

            for &(dx, dy) in directions {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
                if nx >= 0 && nx < height as isize && ny >= 0 && ny < width as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if group_id[nx][ny] == -1 && grid[nx][ny] == char {
                        stack.push((nx, ny)); // Continue DFS
                    }
                }
            }
        }
    }

    // Mark groups in the grid
    for i in 0..height {
        for j in 0..width {
            if group_id[i][j] == -1 {
                dfs(&grid, &mut group_id, i, j, current_group, &directions);
                current_group += 1; // Increment group identifier
            }
        }
    }

    // Count distinct pairs
    let mut group_counts = vec![0; current_group as usize];
    for i in 0..height {
        for j in 0..width {
            group_counts[group_id[i][j] as usize] += 1; // Count cells in each group
        }
    }

    let mut total_pairs = 0;
    let mut group_set = HashSet::new();

    for i in 0..height {
        for j in 0..width {
            let current_group_id = group_id[i][j];
            let current_char = grid[i][j];

            // Count pairs with different groups
            for k in 0..current_group as usize {
                if k != current_group_id as usize {
                    total_pairs += group_counts[k]; // Add counts of other groups
                }
            }
            group_set.insert(current_char); // Track distinct characters
        }
    }

    // Return the total count of pairs
    Ok(total_pairs)
}

fn main() {
    // Example usage
    let height = 3;
    let width = 3;
    let grid = vec![
        vec!['A', 'B', 'A'],
        vec!['B', 'A', 'B'],
        vec!['A', 'B', 'A'],
    ];

    match count_distinct_pairs(height, width, grid) {
        Ok(count) => println!("Total distinct pairs: {}", count),
        Err(e) => eprintln!("Error: {}", e),
    }
}
