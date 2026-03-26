use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut matrix = [[0; 3]; 3];
    let mut min_values = [0; 3];

    // Read input into the matrix
    let mut numbers = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        numbers.extend(line.split_whitespace()
                          .filter_map(|s| s.parse::<i32>().ok()));
    }

    // Ensure we have exactly 9 integers
    if numbers.len() != 9 {
        eprintln!("Invalid input: expected exactly 9 integers.");
        return;
    }

    // Fill the matrix and compute the minimum for each row
    for i in 0..3 {
        for j in 0..3 {
            matrix[i][j] = numbers[i * 3 + j];
        }
        min_values[i] = *matrix[i].iter().min().unwrap(); // Find the minimum in the row
    }

    // Normalize the matrix by subtracting the minimum of each row
    for i in 0..3 {
        for j in 0..3 {
            matrix[i][j] -= min_values[i];
        }
    }

    // Check if all columns are identical
    for j in 0..3 {
        if matrix[0][j] != matrix[1][j] || matrix[0][j] != matrix[2][j] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
