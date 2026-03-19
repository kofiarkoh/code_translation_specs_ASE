use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock(); // Make reader mutable

    loop {
        // Read dimensions X and Y
        let mut dimensions = String::new();
        reader.read_line(&mut dimensions).expect("Failed to read line");
        let dimensions: Vec<i32> = dimensions
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Ensure we have exactly two dimensions
        if dimensions.len() != 2 {
            continue; // Invalid input, skip to next iteration
        }

        let x = dimensions[0];
        let y = dimensions[1];

        // Termination condition
        if x == 0 && y == 0 {
            break;
        }

        // Initialize the course data matrix
        let mut course_data = vec![vec![0; y as usize]; x as usize];

        // Read course data
        for i in 0..x {
            let mut row = String::new();
            reader.read_line(&mut row).expect("Failed to read line");
            let row_data: Vec<i32> = row
                .trim()
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            // Ensure the row matches the expected width
            if row_data.len() != y as usize {
                continue; // Invalid row length, skip to next iteration
            }

            course_data[i as usize] = row_data;
        }

        // Compute the sum of the course data
        let computed_sum: i32 = course_data.iter()
            .flat_map(|row| row.iter())
            .sum();

        // Output the computed sum
        println!("{}", computed_sum);
    }
}
