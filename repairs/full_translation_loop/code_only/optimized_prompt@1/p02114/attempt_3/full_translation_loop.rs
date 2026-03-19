use std::io::{self, BufRead};

const X: [i32; 4] = [1, 0, -1, 0];
const Y: [i32; 4] = [0, 1, 0, -1];

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut d = vec![vec![' '; n + 2]; n + 2]; // Create a (n+2)x(n+2) grid initialized with spaces

    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        for (j, c) in line.chars().enumerate() {
            d[i][j + 1] = c; // Fill the grid starting from (1,1)
        }
    }

    let mut c = 0;
    let mut t = 0;
    let mut x = vec![0; 100010];
    let mut y = vec![0; 100010];

    for i in 0..n {
        for j in 0..n {
            if d[i + 1][j + 1] == 'x' {
                continue;
            }
            d[(i + 1) as usize][(j + 1) as usize] = 'x'; // Mark the starting position
            x[t] = (j + 1) as i32;
            y[t] = (i + 1) as i32;
            c += 1;
            t += 1; // Increment t for the new region
            let mut r = 1;

            while t > 0 {
                t -= 1; // Decrement t to get the current position
                for k in 0..4 {
                    let ny = y[t] + Y[k];
                    let nx = x[t] + X[k];
                    if d[ny as usize][nx as usize] != 'o' {
                        continue;
                    }
                    d[ny as usize][nx as usize] = 'x'; // Mark the new position
                    x[r] = nx;
                    y[r] = ny;
                    r += 1;
                }
            }
        }
    }

    println!("{}", (c - 1) / 3);
}
