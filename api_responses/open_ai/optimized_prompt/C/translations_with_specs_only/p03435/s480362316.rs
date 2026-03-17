use std::io;

fn main() {
    // Create a 3x3 matrix to hold the input integers
    let mut matrix = [[0; 3]; 3];

    // Read 9 integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Populate the matrix with the input values
    for i in 0..3 {
        for j in 0..3 {
            matrix[i][j] = numbers[i * 3 + j];
        }
    }

    // Normalize the matrix by subtracting the minimum of each row
    for i in 0..3 {
        let min_value = *matrix[i].iter().min().unwrap(); // Find the minimum in the row
        for j in 0..3 {
            matrix[i][j] -= min_value; // Subtract the minimum from each element
        }
    }

    // Check if all columns contain identical values
    let mut identical_columns = true;
    for j in 0..3 {
        let first_value = matrix[0][j];
        for i in 1..3 {
            if matrix[i][j] != first_value {
                identical_columns = false; // Found differing values in the column
                break;
            }
        }
        if !identical_columns {
            break; // No need to check further if we already found a difference
        }
    }

    // Output the result
    if identical_columns {
        println!("Yes");
    } else {
        println!("No");
    }
}
