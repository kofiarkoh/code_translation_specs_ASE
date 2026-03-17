use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let w = dimensions.next().unwrap();
    let h = dimensions.next().unwrap();

    let mut d = [[2; 210]; 110]; // Initialize with 2
    let mut x = vec![0; 10000];
    let mut y = vec![0; 10000];
    let mut s = 0;
    let mut r = 1;
    let mut t = 0;

    // Read the grid
    for i in 1..=h {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        for j in 1..=w {
            d[i as usize][(j * 2 + (i % 2)) as usize] = values[(j - 1) as usize];
        }
    }

    // Directions
    let x_offsets = [1, 2, 1, -1, -2, -1];
    let y_offsets = [1, 0, -1, -1, 0, 1];

    while r - t > 0 {
        for i in 0..6 {
            let nx = x[t] + x_offsets[i];
            let ny = y[t] + y_offsets[i];
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

    // Count the valid neighbors
    for i in 1..=h {
        for j in 1..=w {
            if d[i as usize][(j * 2 + (i % 2)) as usize] - 1 != 0 {
                continue;
            }
            for k in 0..6 {
                if d[(i + y_offsets[k]) as usize][(j * 2 + (i % 2) + x_offsets[k]) as usize] == 2 {
                    s += 1;
                }
            }
        }
    }

    println!("{}", s);
}
