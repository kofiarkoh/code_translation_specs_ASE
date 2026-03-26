use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read width and height
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let w: usize = dimensions.next().unwrap().parse().unwrap();
    let h: usize = dimensions.next().unwrap().parse().unwrap();

    // Initialize the grid
    let mut d = vec![vec![0; w * 2 + 4]; h + 2];

    // Read the grid values
    for i in 1..=h {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 1..=w {
            d[i][j * 2 + i % 2] = values[j - 1];
        }
    }

    let mut x = vec![0; 10000];
    let mut y = vec![0; 10000];
    let mut t = 0;
    let mut r = 1; // Start r at 1 since x[0] and y[0] are initialized
    let mut s = 0;

    // Directions for movement
    let X = [1, 2, 1, -1, -2, -1];
    let Y = [1, 0, -1, -1, 0, 1];

    // Mark the initial position
    x[0] = 0;
    y[0] = 0;
    d[0][0] = 2; // Mark the initial position as processed

    while t < r {
        for i in 0..6 {
            let nx = x[t] + X[i];
            let ny = y[t] + Y[i];

            // Check bounds
            if nx < 0 || ny < 0 || ny >= h as isize + 1 || nx >= w as isize * 2 + 4 {
                continue;
            }
            if d[ny as usize][nx as usize] != 0 {
                continue;
            }

            d[ny as usize][nx as usize] = 2;
            x[r] = nx;
            y[r] = ny;
            r += 1;
        }
        t += 1;
    }

    // Count unmarked cells adjacent to marked cells
    for i in 1..=h {
        for j in 1..=w {
            if d[i][j * 2 + i % 2] != 1 {
                continue;
            }
            for k in 0..6 {
                let adj_x = j * 2 + i % 2 + X[k];
                let adj_y = i + Y[k];
                if adj_y >= 0 && adj_y < d.len() && adj_x >= 0 && adj_x < d[0].len() {
                    if d[adj_y as usize][adj_x as usize] == 2 {
                        s += 1;
                    }
                }
            }
        }
    }

    // Output the result
    println!("{}", s);
}
