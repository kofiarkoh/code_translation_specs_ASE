use std::io::{self, BufRead};

const X: [isize; 4] = [1, 0, -1, 0];
const Y: [isize; 4] = [0, 1, 0, -1];

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the size of the grid
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input");

    // Initialize the grid with an extra border
    let mut grid = vec![vec![' '; n + 2]; n + 2];

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
                for k in 0..4 {
                    let ny = y as isize + Y[k];
                    let nx = x as isize + X[k];

                    // Check bounds and if the cell is the specified character
                    if grid[ny as usize][nx as usize] == 'o' {
                        grid[ny as usize][nx as usize] = 'x'; // Mark as visited
                        stack.push((ny as usize, nx as usize));
                    }
                }
            }
        }
    }

    // Adjust the count according to the specification
    let adjusted_count = (count as isize - 1) / 3;
    println!("{}", adjusted_count);
}
