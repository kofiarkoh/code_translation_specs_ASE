use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut matrix = [[0; 3]; 3];
    let mut min = [0; 3];

    // Read input into the matrix
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input.split_whitespace()
                                  .filter_map(|s| s.parse().ok())
                                  .collect();

    // Check if we have exactly 9 integers
    if numbers.len() != 9 {
        eprintln!("Invalid input: expected exactly 9 integers.");
        std::process::exit(1);
    }

    // Fill the matrix and compute the minimum of each row
    for i in 0..3 {
        for j in 0..3 {
            matrix[i][j] = numbers[i * 3 + j];
        }
        min[i] = *matrix[i].iter().min().unwrap(); // Safe as we know there are 3 elements
    }

    // Normalize the rows by subtracting the minimum
    for i in 0..3 {
        for j in 0..3 {
            matrix[i][j] -= min[i];
        }
    }

    // Check if all columns are identical
    for j in 0..3 {
        if matrix[0][j] != matrix[1][j] || matrix[0][j] != matrix[2][j] {
            println!("No");
            std::process::exit(0);
        }
    }

    println!("Yes");
}
