use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let w: usize = dimensions.next().unwrap().parse().unwrap();
    let h: usize = dimensions.next().unwrap().parse().unwrap();

    let mut d = [[2; 210]; 110]; // Initialize with 2
    for i in 1..=h {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for j in 1..=w {
            d[i][j * 2 + i % 2] = values[j - 1];
        }
    }

    let mut x = vec![0; 10000];
    let mut y = vec![0; 10000];
    let mut t = 0;
    let mut r = 1;
    let mut s = 0;

    let X = [1, 2, 1, -1, -2, -1]; // Use i32 for negative values
    let Y = [1, 0, -1, -1, 0, 1]; // Use i32 for negative values

    while r - t > 0 {
        for i in 0..6 {
            let nx = x[t] + X[i];
            let ny = y[t] + Y[i];
            if nx < 0 || ny < 0 || ny > h + 1 || nx > w * 2 + 4 {
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

    for i in 1..=h {
        for j in 1..=w {
            if d[i][j * 2 + i % 2] - 1 != 0 {
                continue;
            }
            for k in 0..6 {
                if d[(i as i32 + Y[k]) as usize][(j * 2 + i % 2 + X[k]) as usize] == 2 {
                    s += 1;
                }
            }
        }
    }

    println!("{}", s);
}
