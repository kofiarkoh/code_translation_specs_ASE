use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Read each line of input
        let line = line.expect("Failed to read line");
        let mut numbers = line.split_whitespace()
            .filter_map(|s| s.parse::<f64>().ok());

        // Collect six double-precision floating-point numbers
        let a = match numbers.next() {
            Some(a) if a != 0.0 => a, // Ensure a is not zero to avoid division by zero
            _ => continue, // Skip if a is zero or not provided
        };
        let b = numbers.next().unwrap_or(0.0);
        let c = numbers.next().unwrap_or(0.0);
        let d = numbers.next().unwrap_or(0.0);
        let e = numbers.next().unwrap_or(0.0);
        let f = numbers.next().unwrap_or(0.0);

        // Calculate x and y based on the equations derived from the inputs
        let x = (c - b * f / d) / (a - b * e / d);
        let y = (f - e * x) / d;

        // Output the results formatted to three decimal places
        println!("{:.3} {:.3}", x, y);
    }
}
