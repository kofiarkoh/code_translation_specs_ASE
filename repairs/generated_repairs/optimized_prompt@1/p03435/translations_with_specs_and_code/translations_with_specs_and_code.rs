use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the input into a 3x3 matrix
    let mut matrix = [[0; 3]; 3];
    let mut values = input.split_whitespace().filter_map(|s| s.parse::<i32>().ok());
    
    for i in 0..3 {
        for j in 0..3 {
            matrix[i][j] = values.next().expect("Expected exactly 9 integers");
        }
    }

    // Compute the minimum value for each row
    let mut min = [0; 3];
    for i in 0..3 {
        min[i] = *matrix[i].iter().min().expect("Row should not be empty");
    }

    // Normalize the matrix by subtracting the row minimums
    for i in 0..3 {
        for j in 0..3 {
            matrix[i][j] -= min[i];
        }
    }

    // Check if all columns in the normalized matrix are equal
    for j in 0..3 {
        if matrix[0][j] != matrix[1][j] || matrix[0][j] != matrix[2][j] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
