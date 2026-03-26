use std::io::{self, BufRead};

const MAX_SIZE: usize = 300;
const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the size of the grid
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    assert!(n > 0 && n <= MAX_SIZE, "n must be between 1 and 300");

    // Initialize the grid
    let mut grid = vec![vec![' '; MAX_SIZE + 2]; MAX_SIZE + 2];

    // Read the grid characters
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        for (j, c) in line.chars().enumerate() {
            grid[i][j + 1] = c;
        }
    }

    let mut component_count = 0;

    // Function to perform DFS and mark connected 'o's
    fn dfs(grid: &mut Vec<Vec<char>>, x: usize, y: usize, n: usize) {
        let mut stack = vec![(x, y)];
        grid[x][y] = 'x'; // Mark as visited

        while let Some((cx, cy)) = stack.pop() {
            for &(dx, dy) in &DIRECTIONS {
                let nx = (cx as isize + dx) as usize;
                let ny = (cy as isize + dy) as usize;

                if nx > 0 && nx <= n && ny > 0 && ny <= n && grid[nx][ny] == 'o' {
                    grid[nx][ny] = 'x'; // Mark as visited
                    stack.push((nx, ny));
                }
            }
        }
    }

    // Count distinct components
    for i in 1..=n {
        for j in 1..=n {
            if grid[i][j] == 'o' {
                dfs(&mut grid, i, j, n);
                component_count += 1;
            }
        }
    }

    // Output the result
    println!("{}", (component_count - 1) / 3);
}
