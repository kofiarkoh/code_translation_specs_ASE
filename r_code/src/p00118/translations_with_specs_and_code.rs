use std::io::{self, BufRead};

const MAX: usize = 100;

fn divide(farm: &mut [[char; MAX]; MAX], i: usize, j: usize, h: usize, w: usize, moji: char) {
    if farm[i][j] == moji {
        farm[i][j] = '+'; // Mark the cell as processed
        if j + 1 < w {
            divide(farm, i, j + 1, h, w, moji);
        }
        if i + 1 < h {
            divide(farm, i + 1, j, h, w, moji);
        }
        if j > 0 {
            divide(farm, i, j - 1, h, w, moji);
        }
        if i > 0 {
            divide(farm, i - 1, j, h, w, moji);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut farm: [[char; MAX]; MAX] = [[' '; MAX]; MAX]; // Initialize the farm grid
    let mut input = reader.lines();

    loop {
        let line = input.next().unwrap().unwrap();
        let dimensions: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let h = dimensions[0];
        let w = dimensions[1];

        if h == 0 && w == 0 {
            break; // Termination condition
        }

        for i in 0..h {
            let line = input.next().unwrap().unwrap();
            for (j, c) in line.chars().enumerate() {
                farm[i][j] = c; // Fill the farm grid
            }
        }

        let mut cnt = 0;
        for i in 0..h {
            for j in 0..w {
                if farm[i][j] == '+' {
                    continue; // Skip already processed cells
                }
                cnt += 1; // Found a new region
                divide(&mut farm, i, j, h, w, farm[i][j]);
            }
        }
        println!("{}", cnt); // Output the count of regions
    }
}
