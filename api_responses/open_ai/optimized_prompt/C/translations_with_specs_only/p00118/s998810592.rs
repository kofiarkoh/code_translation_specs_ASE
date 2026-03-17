fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let dimensions: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Read height and width
        let height = dimensions[0];
        let width = dimensions[1];

        // Termination condition
        if height == 0 && width == 0 {
            break;
        }

        // Initialize the grid
        let mut grid: Vec<Vec<char>> = Vec::with_capacity(height);
        for _ in 0..height {
            let mut grid_line = String::new();
            reader.read_line(&mut grid_line).unwrap();
            grid.push(grid_line.trim().chars().collect());
        }

        // Call the function to count distinct areas
        let area_count = count_distinct_areas(&grid, height, width);
        println!("{}", area_count);
    }
}

// Function to count distinct connected areas of identical characters
fn count_distinct_areas(grid: &[Vec<char>], height: usize, width: usize) -> usize {
    let mut area_id = vec![vec![0; width]; height]; // Tracks area identifiers
    let mut area_num = 0; // Counts distinct areas

    // Directions for moving up, down, left, right
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Helper function for depth-first search
    fn dfs(grid: &[Vec<char>], area_id: &mut Vec<Vec<usize>>, x: usize, y: usize, area_num: usize, height: usize, width: usize) {
        // Mark the current cell with the area number
        area_id[x][y] = area_num;

        // Explore neighbors
        for &(dx, dy) in &directions {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            // Check bounds and connectivity
            if new_x >= 0 && new_x < height as isize && new_y >= 0 && new_y < width as isize {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if area_id[new_x][new_y] == 0 && grid[new_x][new_y] == grid[x][y] {
                    dfs(grid, area_id, new_x, new_y, area_num, height, width);
                }
            }
        }
    }

    // Iterate through each cell in the grid
    for i in 0..height {
        for j in 0..width {
            // If the cell is not visited, start a new area
            if area_id[i][j] == 0 {
                area_num += 1; // Increment area count
                dfs(grid, &mut area_id, i, j, area_num, height, width);
            }
        }
    }

    area_num // Return the count of distinct areas
}
