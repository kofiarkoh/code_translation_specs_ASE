use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read width and height
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let w: usize = dimensions.next().unwrap().parse().unwrap();
    let h: usize = dimensions.next().unwrap().parse().unwrap();

    // Initialize the grid
    let mut d = vec![vec![0; 2 * w + 4]; h + 2];

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

    // Movement vectors
    let x_moves = [1, 2, 1, -1, -2, -1];
    let y_moves = [1, 0, -1, -1, 0, 1];

    let mut x = vec![0; 10000];
    let mut y = vec![0; 10000];
    let mut r = 1;
    let mut t = 0;
    let mut s = 0;

    // Mark cells based on traversal
    while t < r {
        for i in 0..6 {
            let nx = x[t] as isize + x_moves[i];
            let ny = y[t] as isize + y_moves[i];

            // Check bounds
            if nx < 0 || ny < 0 || ny > h as isize + 1 || nx >= 2 * w as isize + 4 {
                continue;
            }
            if d[ny as usize][nx as usize] != 0 {
                continue;
            }

            d[ny as usize][nx as usize] = 2;
            x.push(nx as usize);
            y.push(ny as usize);
            r += 1;
        }
        t += 1;
    }

    // Count adjacent cells
    for i in 1..=h {
        for j in 1..=w {
            if d[i][j * 2 + i % 2] != 1 {
                continue;
            }
            for k in 0..6 {
                if d[i + y_moves[k]][j * 2 + i % 2 + x_moves[k]] == 2 {
                    s += 1;
                }
            }
        }
    }

    // Output the result
    println!("{}", s);
}
