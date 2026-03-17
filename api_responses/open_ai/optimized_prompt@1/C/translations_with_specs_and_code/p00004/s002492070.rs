use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<f64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Ensure we have exactly six values
        if values.len() != 6 {
            continue; // Skip invalid input
        }

        let (a, b, c, d, e, f) = (values[0], values[1], values[2], values[3], values[4], values[5]);

        // Check the precondition: the first input value must be non-zero
        if a == 0.0 {
            // Division by zero would occur, so we skip this iteration
            continue; // No explicit error handling as per the spec
        }

        let bai = d / a;
        let aa = a * bai;
        let bb = b * bai;
        let cc = c * bai;

        let y = (f - cc) / (e - bb);
        let x = (c - b * y) / a;

        // Output formatted to three decimal places
        println!("{:.3} {:.3}", x, y);
    }
}
