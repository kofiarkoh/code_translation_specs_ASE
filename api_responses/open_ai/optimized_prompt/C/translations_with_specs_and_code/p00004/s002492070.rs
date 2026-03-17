use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap the Result to get the String
        let values: Vec<&str> = line.split_whitespace().collect();

        // Ensure we have exactly 6 values
        if values.len() != 6 {
            continue; // Skip this iteration if not enough values
        }

        // Parse the input values as f64
        let a: f64 = match values[0].parse() {
            Ok(val) => val,
            Err(_) => continue, // Skip if parsing fails
        };

        // Check for the precondition: the first input must be non-zero
        if a == 0.0 {
            continue; // Skip this iteration to avoid division by zero
        }

        let b: f64 = values[1].parse().unwrap_or(0.0);
        let c: f64 = values[2].parse().unwrap_or(0.0);
        let d: f64 = values[3].parse().unwrap_or(0.0);
        let e: f64 = values[4].parse().unwrap_or(0.0);
        let f: f64 = values[5].parse().unwrap_or(0.0);

        // Perform calculations
        let bai = d / a;
        let aa = a * bai;
        let bb = b * bai;
        let cc = c * bai;

        // Calculate y and x
        let y = (f - cc) / (e - bb);
        let x = (c - b * y) / a;

        // Output the results formatted to three decimal places
        println!("{:.3} {:.3}", x, y);
    }
}
