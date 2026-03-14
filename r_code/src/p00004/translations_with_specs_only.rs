use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let coefficients: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Ensure we have exactly six coefficients
        if coefficients.len() != 6 {
            continue; // Undefined behavior for incorrect input, so we skip
        }

        // Unpack coefficients
        let (a1, b1, c1, a2, b2, c2) = (
            coefficients[0],
            coefficients[1],
            coefficients[2],
            coefficients[3],
            coefficients[4],
            coefficients[5],
        );

        // Calculate the determinant
        let determinant = a1 * b2 - a2 * b1;

        // Initialize solutions
        let (mut x, mut y) = (0.0, 0.0);

        // Check for division by zero (determinant)
        if determinant != 0 {
            // Compute the solutions
            x = (c1 * b2 - c2 * b1) as f64 / determinant as f64;
            y = (a1 * c2 - a2 * c1) as f64 / determinant as f64;
        }

        // Adjust to zero if within a specific range (assumed to be close to zero)
        if x.abs() < 1e-6 {
            x = 0.0;
        }
        if y.abs() < 1e-6 {
            y = 0.0;
        }

        // Output the results formatted to three decimal places
        println!("{:.3} {:.3}", x, y);
    }
}
