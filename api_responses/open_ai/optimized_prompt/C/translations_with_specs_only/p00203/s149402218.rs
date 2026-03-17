use std::io::{self, BufRead};

fn main() {
    // Read dimensions X and Y from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut dimensions = String::new();
    
    // Read the first line for dimensions
    reader.read_line(&mut dimensions).expect("Failed to read line");
    let dimensions: Vec<i32> = dimensions
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have exactly two dimensions
    if dimensions.len() != 2 {
        eprintln!("Invalid input: Expected two dimensions.");
        return;
    }

    let x = dimensions[0];
    let y = dimensions[1];

    // Check for termination condition
    if x == 0 && y == 0 {
        return; // Exit without further processing
    }

    // Initialize a 2D vector to store course values
    let mut course_data = vec![vec![0; y as usize]; x as usize];

    // Read course values
    for i in 0..x {
        let mut values = String::new();
        reader.read_line(&mut values).expect("Failed to read line");
        let row_values: Vec<i32> = values
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Ensure the number of values matches the dimension Y
        if row_values.len() != y as usize {
            eprintln!("Invalid input: Expected {} values for row {}", y, i);
            return;
        }

        // Store the values in the course data structure
        course_data[i as usize] = row_values;
    }

    // Compute the sum of specific patterns (for simplicity, we sum all values)
    let sum: i32 = course_data.iter().flat_map(|row| row.iter()).sum();

    // Output the computed sum
    println!("{}", sum);
}
