use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut matrix = [[0; 3]; 3];
    
    // Read exactly 9 integers into the 3x3 matrix
    let mut input_numbers = Vec::new();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            input_numbers.extend(line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()));
        }
    }

    // Ensure we have exactly 9 integers
    if input_numbers.len() != 9 {
        // Behavior is not defined for invalid input, so we do nothing
        return;
    }

    // Populate the matrix
    for i in 0..3 {
        for j in 0..3 {
            matrix[i][j] = input_numbers[i * 3 + j];
        }
    }

    // Compute the minimum for each row
    let mut min = [0; 3];
    for i in 0..3 {
        min[i] = *matrix[i].iter().min().unwrap(); // Safe since we know the array has 3 elements
    }

    // Normalize the matrix by subtracting the row minimums
    for i in 0..3 {
        for j in 0..3 {
            matrix[i][j] -= min[i];
        }
    }

    // Check if all columns are equal
    for j in 0..3 {
        if matrix[0][j] != matrix[1][j] || matrix[0][j] != matrix[2][j] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
