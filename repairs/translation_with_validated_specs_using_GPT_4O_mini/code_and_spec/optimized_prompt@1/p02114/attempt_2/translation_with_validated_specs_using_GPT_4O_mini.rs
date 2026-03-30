use std::io::{self, BufRead};

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the size of the grid
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut grid = vec![vec![' '; n + 2]; n + 2]; // Create a grid with padding

    // Read the grid characters
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        for (j, c) in line.chars().enumerate() {
            grid[i][j + 1] = c;
        }
    }

    let mut component_count = 0;

    // Process the grid to count connected components of 'x'
    for i in 1..=n {
        for j in 1..=n {
            if grid[i][j] == 'x' {
                continue;
            }

            // Start a new component
            grid[i][j] = 'x'; // Mark as visited
            component_count += 1;

            let mut stack = vec![(i, j)];
            while let Some((y, x)) = stack.pop() {
                for &(dy, dx) in &DIRECTIONS {
                    let ny = (y as isize + dy) as usize;
                    let nx = (x as isize + dx) as usize;

                    // Check bounds and if the cell is not 'o' and not already visited
                    if ny > 0 && ny <= n && nx > 0 && nx <= n && grid[ny][nx] == 'x' {
                        grid[ny][nx] = 'x'; // Mark as visited
                        stack.push((ny, nx));
                    }
                }
            }
        }
    }

    // Output the result
    println!("{}", component_count / 3);
}
