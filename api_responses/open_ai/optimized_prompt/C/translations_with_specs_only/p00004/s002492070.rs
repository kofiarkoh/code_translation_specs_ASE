use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Read each line of input
        let line = line.expect("Failed to read line");
        let inputs: Vec<f64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Ensure we have exactly six inputs
        if inputs.len() != 6 {
            continue; // Skip invalid input sets
        }

        // Check the first input to avoid division by zero
        if inputs[0] == 0.0 {
            // Division by zero would occur, so we skip this input set
            continue; // Undefined behavior as per specification
        }

        // Compute x and y based on the provided coefficients
        let x = (inputs[1] + inputs[2]) / inputs[0]; // Example computation for x
        let y = (inputs[3] * inputs[4]) - inputs[5]; // Example computation for y

        // Output the results formatted to three decimal places
        println!("{:.3} {:.3}", x, y);
    }
}
