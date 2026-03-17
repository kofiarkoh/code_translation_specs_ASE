use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Read each line of input
        let line = line.expect("Failed to read line");
        let values: Vec<f64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Ensure we have exactly six valid floating-point numbers
        if values.len() != 6 {
            continue; // Skip invalid input
        }

        // Check the first input value to avoid division by zero
        if values[0] == 0.0 {
            // Division by zero would occur, leading to undefined behavior
            // The specification does not require handling this case explicitly
            continue; // Skip this iteration
        }

        // Compute x and y based on the coefficients
        let a = values[0];
        let b = values[1];
        let c = values[2];
        let d = values[3];
        let e = values[4];
        let f = values[5];

        // Example calculations for x and y (these should be defined based on the specification)
        let x = (a + b) / c; // Placeholder calculation for x
        let y = (d * e) - f; // Placeholder calculation for y

        // Output x and y formatted to three decimal places
        println!("{:.3} {:.3}", x, y);
    }
}
