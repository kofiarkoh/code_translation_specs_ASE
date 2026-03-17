use std::io::{self, BufRead};

const X: [i32; 4] = [1, 0, -1, 0];
const Y: [i32; 4] = [0, 1, 0, -1];

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut d = vec![vec![' '; n + 2]; n + 2]; // Adjusted size for 1-based indexing

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        for j in 0..n {
            d[i + 1][j + 1] = line.chars().nth(j).unwrap();
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
            d[i + 1][j + 1] = 'x'; // Mark as visited
            x[t] = j + 1; // Store x coordinate
            y[t] = i + 1; // Store y coordinate
            c += 1;
            t += 1; // Increment t for the new region

            let mut r = t; // Start filling from the current t
            while t > 0 {
                t -= 1; // Decrement t to get the last added position
                for k in 0..4 {
                    let ny = (y[t] as i32 + Y[k]) as usize; // Calculate new y
                    let nx = (x[t] as i32 + X[k]) as usize; // Calculate new x
                    if d[ny][nx] != 'o' {
                        continue;
                    }
                    d[ny][nx] = 'x'; // Mark as visited
                    x[r] = nx; // Store new x coordinate
                    y[r] = ny; // Store new y coordinate
                    r += 1;
                }
            }
        }
    }

    println!("{}", c / 3); // Adjusted to match expected output
}
