use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input: Vec<i32> = stdin.lock().lines()
        .flat_map(|line| line.unwrap().split_whitespace().filter_map(|s| s.parse().ok()))
        .collect();

    // Check if we have exactly 9 integers
    if input.len() != 9 {
        // Undefined behavior as per specification; we do not handle errors
        return;
    }

    // Create a 3x3 matrix from the input
    let matrix = [
        [input[0], input[1], input[2]],
        [input[3], input[4], input[5]],
        [input[6], input[7], input[8]],
    ];

    // Compute the minimum value for each row
    let row_mins: [i32; 3] = [
        *matrix[0].iter().min().unwrap(),
        *matrix[1].iter().min().unwrap(),
        *matrix[2].iter().min().unwrap(),
    ];

    // Normalize the matrix by subtracting the row minimums
    let normalized: [[i32; 3]; 3] = [
        [matrix[0][0] - row_mins[0], matrix[0][1] - row_mins[0], matrix[0][2] - row_mins[0]],
        [matrix[1][0] - row_mins[1], matrix[1][1] - row_mins[1], matrix[1][2] - row_mins[1]],
        [matrix[2][0] - row_mins[2], matrix[2][1] - row_mins[2], matrix[2][2] - row_mins[2]],
    ];

    // Check if all columns in the normalized matrix are equal
    let all_columns_equal = (normalized[0][0] == normalized[1][0] && normalized[1][0] == normalized[2][0]) ||
                            (normalized[0][1] == normalized[1][1] && normalized[1][1] == normalized[2][1]) ||
                            (normalized[0][2] == normalized[1][2] && normalized[1][2] == normalized[2][2]);

    // Output the result
    if all_columns_equal {
        println!("Yes");
    } else {
        println!("No");
    }
}
