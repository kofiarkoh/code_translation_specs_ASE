use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut c = [[0; 3]; 3];
    let mut min = [0; 3];

    // Read input into the 2D array
    for i in 0..3 {
        let line = stdin.lock().lines().next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..3 {
            c[i][j] = nums[j];
        }

        // Determine the minimum value in the row
        if c[i][0] > c[i][1] {
            if c[i][1] > c[i][2] {
                min[i] = c[i][2];
            } else {
                min[i] = c[i][1];
            }
        } else {
            if c[i][0] > c[i][2] {
                min[i] = c[i][2];
            } else {
                min[i] = c[i][0];
            }
        }
    }

    // Subtract the minimum value from each element in the row
    for i in 0..3 {
        for j in 0..3 {
            c[i][j] -= min[i];
        }
    }

    // Check if all columns are equal
    for j in 0..3 {
        if c[0][j] != c[1][j] || c[0][j] != c[2][j] {
            println!("No");
            return; // Exit the program
        }
    }

    println!("Yes");
}
