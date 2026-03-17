use std::io;

fn main() {
    let mut matrix = [[0; 3]; 3];
    let mut min_values = [0; 3];

    // Read input values into the matrix
    for i in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let values: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();

        for j in 0..3 {
            matrix[i][j] = values[j];
        }

        // Find the minimum value in the current row
        min_values[i] = *matrix[i].iter().min().expect("Row should not be empty");
    }

    // Normalize the matrix by subtracting the minimum from each element in the row
    for i in 0..3 {
        for j in 0..3 {
            matrix[i][j] -= min_values[i];
        }
    }

    // Check if all columns contain identical values
    for j in 0..3 {
        if matrix[0][j] != matrix[1][j] || matrix[0][j] != matrix[2][j] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
