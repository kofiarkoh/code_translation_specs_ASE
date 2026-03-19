use std::io::{self, BufRead};

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the size of the grid
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input size");

    // Initialize the grid
    let mut grid = vec![vec![' '; n + 2]; n + 2]; // Padding to avoid boundary checks

    // Read the grid characters
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        for (j, c) in line.chars().enumerate() {
            grid[i][j + 1] = c;
        }
    }

    let mut count = 0;

    // Process the grid to count distinct connected components
    for i in 1..=n {
        for j in 1..=n {
            if grid[i][j] == 'x' {
                continue;
            }

            // Start a new component
            grid[i][j] = 'x'; // Mark as visited
            count += 1;

            let mut stack = vec![(i, j)];
            while let Some((y, x)) = stack.pop() {
                for &(dy, dx) in &DIRECTIONS {
                    let ny = (y as isize + dy) as usize;
                    let nx = (x as isize + dx) as usize;

                    if grid[ny][nx] == 'o' {
                        grid[ny][nx] = 'x'; // Mark as visited
                        stack.push((ny, nx));
                    }
                }
            }
        }
    }

    // Output the adjusted count of distinct connected components
    println!("{}", (count as isize - 1) / 3);
}
