use std::io::{self, BufRead};

const X: [i32; 4] = [1, 0, -1, 0];
const Y: [i32; 4] = [0, 1, 0, -1];

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut d = vec![vec![' '; 302]; 302]; // 300 + 2 for 1-based indexing

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
            d[y[t] = i + 1][x[0] = j + 1] = 'x';
            c += 1;
            t = 0; // Reset t for the new region

            let mut r = 1; // Start filling from index 1
            while t < r {
                for k in 0..4 {
                    let ny = y[t] + Y[k];
                    let nx = x[t] + X[k];
                    if d[ny][nx] != 'o' {
                        continue;
                    }
                    d[y[r] = ny][x[r] = nx] = 'x';
                    r += 1;
                }
                t += 1;
            }
        }
    }

    println!("{}", (c - 1) / 3);
}
