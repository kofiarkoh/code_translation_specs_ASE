use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the grid
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Initialize the grid
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        grid.push(line.chars().collect());
    }

    // Count distinct connected components of 'o'
    let mut component_count = 0;

    // Directions for moving in the grid (up, down, left, right)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Function to perform DFS
    fn dfs(grid: &mut Vec<Vec<char>>, x: usize, y: usize, n: usize, directions: &[(isize, isize)]) {
        if x >= n || y >= n || grid[x][y] != 'o' {
            return;
        }
        // Mark the cell as visited
        grid[x][y] = 'x';
        // Explore all four directions
        for &(dx, dy) in directions {
            dfs(grid, (x as isize + dx) as usize, (y as isize + dy) as usize, n, directions);
        }
    }

    // Iterate through the grid
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 'o' {
                // Found a new component
                component_count += 1;
                // Perform DFS to mark all connected 'o's
                dfs(&mut grid, i, j, n, &directions);
            }
        }
    }

    // Output the number of distinct components divided by three
    println!("{}", component_count / 3);
}
