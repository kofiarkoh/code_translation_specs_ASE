use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut values: Vec<f64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Ensure we have exactly six valid floating-point numbers
        if values.len() != 6 {
            continue; // Skip invalid input
        }

        // Check the first input value to avoid division by zero
        if values[0] == 0.0 {
            // Division by zero would occur; undefined behavior as per specification
            continue; // Skip this iteration
        }

        // Compute x and y based on the coefficients
        let x = (values[1] * values[2]) / values[0]; // Corrected computation for x
        let y = (values[3] + values[4]) - values[5]; // Corrected computation for y

        // Output x and y formatted to three decimal places
        println!("{:.3} {:.3}", x, y);
    }
}
