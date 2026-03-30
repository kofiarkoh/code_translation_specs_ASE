use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input: Vec<i32> = stdin.lock().lines()
        .filter_map(|line| line.ok()) // Filter out any errors while reading lines
        .flat_map(|line| line.split_whitespace().filter_map(|s| s.parse::<i32>().ok())) // Parse integers safely
        .collect();

    // Check if the input has exactly 9 integers
    if input.len() != 9 {
        return; // Terminate with no output if input is invalid
    }

    // Create a 3x3 matrix from the input
    let matrix = [
        [input[0], input[1], input[2]],
        [input[3], input[4], input[5]],
        [input[6], input[7], input[8]],
    ];

    // Array to hold the minimum values of each row
    let mut row_mins = [0; 3];

    // Compute the minimum value of each row
    for i in 0..3 {
        row_mins[i] = *matrix[i].iter().min().unwrap();
    }

    // Normalize the matrix by subtracting the row minimums
    let mut normalized = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            normalized[i][j] = matrix[i][j] - row_mins[i];
        }
    }

    // Check if all columns in the normalized matrix are identical
    let mut identical_columns = true;
    for j in 0..3 {
        for i in 1..3 {
            if normalized[i][j] != normalized[0][j] {
                identical_columns = false;
                break;
            }
        }
        if !identical_columns {
            break;
        }
    }

    // Output the result
    if identical_columns {
        println!("Yes");
    } else {
        println!("No");
    }
}
