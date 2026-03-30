use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Read a line of input
        let line = line.unwrap();
        // Split the line into parts and parse them as floating-point numbers
        let numbers: Vec<f64> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<f64>().ok())
            .collect();

        // Ensure we have exactly six valid floating-point numbers
        if numbers.len() != 6 {
            continue; // Skip this input set if not valid
        }

        let (a, b, c, d, e, f) = (numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5]);

        // Check the precondition: a must be non-zero
        if a == 0.0 {
            continue; // Skip this input set if a is zero to avoid division by zero
        }

        // Compute the derived values
        let bai = d / a;
        let aa = a * bai;
        let bb = b * bai;
        let cc = c * bai;
        let y = (f - cc) / (e - bb);
        let x = (c - b * y) / a;

        // Output the results formatted to three decimal places
        println!("{:.3} {:.3}", x, y);
    }
}
