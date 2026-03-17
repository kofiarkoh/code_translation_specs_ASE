use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the input into a vector of integers
    let numbers: Vec<i32> = input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Check if we have exactly 9 integers
    if numbers.len() != 9 {
        // Undefined behavior as per specification; we will just exit
        std::process::exit(1);
    }

    // Create a 3x3 matrix from the input
    let mut matrix = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            matrix[i][j] = numbers[i * 3 + j];
        }
    }

    // Normalize each row by subtracting the minimum value of that row
    for i in 0..3 {
        let min_value = *matrix[i].iter().min().unwrap();
        for j in 0..3 {
            matrix[i][j] -= min_value;
        }
    }

    // Check if all columns are identical
    let first_column = (matrix[0][0], matrix[1][0], matrix[2][0]);
    let all_columns_identical = (matrix[0][1], matrix[1][1], matrix[2][1]) == first_column &&
                                 (matrix[0][2], matrix[1][2], matrix[2][2]) == first_column;

    // Output the result
    if all_columns_identical {
        println!("Yes");
        std::process::exit(0);
    } else {
        println!("No");
        std::process::exit(1);
    }
}
