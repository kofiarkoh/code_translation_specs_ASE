use std::io::{self, BufRead};

const MAX_SIZE: usize = 300;
const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the size of the grid
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Initialize the grid
    let mut grid = vec![vec![' '; MAX_SIZE]; MAX_SIZE];
    
    // Read the grid
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        for j in 0..n {
            grid[i][j] = line.chars().nth(j).unwrap();
        }
    }

    let mut component_count = 0;

    // Process the grid to count distinct components
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 'x' {
                continue;
            }
            // Start a new component
            grid[i][j] = 'x'; // Mark as visited
            component_count += 1;

            // Use a stack to perform DFS
            let mut stack = vec![(i, j)];
            while let Some((y, x)) = stack.pop() {
                for &(dy, dx) in &DIRECTIONS {
                    let ny = (y as isize + dy) as usize;
                    let nx = (x as isize + dx) as usize;
                    if ny < n && nx < n && grid[ny][nx] == 'o' {
                        grid[ny][nx] = 'x'; // Mark as visited
                        stack.push((ny, nx));
                    }
                }
            }
        }
    }

    // Output the result
    println!("{}", (component_count - 1) / 3);
}
