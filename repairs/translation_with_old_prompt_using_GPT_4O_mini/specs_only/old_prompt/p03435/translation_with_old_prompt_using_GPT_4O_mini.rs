use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input: Vec<i32> = stdin.lock().lines()
        .filter_map(|line| line.ok()) // Filter out any errors while reading lines
        .flat_map(|line| line.split_whitespace().filter_map(|s| s.parse::<i32>().ok())) // Parse integers safely
        .collect();

    // Ensure we have exactly 9 integers
    if input.len() != 9 {
        // Undefined behavior as per specification; we will just return
        return;
    }

    // Create a 3x3 matrix from the input
    let mut matrix = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            matrix[i][j] = input[i * 3 + j];
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

    // Output result
    if all_columns_identical {
        println!("Yes");
        std::process::exit(0); // Exit with code 0
    } else {
        println!("No");
        std::process::exit(1); // Exit with code 1
    }
}
