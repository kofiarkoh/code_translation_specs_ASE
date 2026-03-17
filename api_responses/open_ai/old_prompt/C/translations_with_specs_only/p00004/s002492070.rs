use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Read each line of input
        let line = line.expect("Failed to read line");
        let mut values = line.split_whitespace()
            .filter_map(|s| s.parse::<f64>().ok()); // Parse to f64, ignore invalid inputs

        // Collect six double values
        let a = match values.next() {
            Some(v) => v,
            None => continue, // Skip if not enough values
        };
        let b = match values.next() {
            Some(v) => v,
            None => continue,
        };
        let c = match values.next() {
            Some(v) => v,
            None => continue,
        };
        let d = match values.next() {
            Some(v) => v,
            None => continue,
        };
        let e = match values.next() {
            Some(v) => v,
            None => continue,
        };
        let f = match values.next() {
            Some(v) => v,
            None => continue,
        };

        // Check for division by zero
        if a == 0.0 {
            // Undefined behavior as per specification; we will just skip this case
            continue;
        }

        // Calculate x and y based on the equations derived from the inputs
        let x = (c - b * (f - d) / a) / (b - a * d / b);
        let y = (f - d * x) / b;

        // Output the results formatted to three decimal places
        println!("{:.3} {:.3}", x, y);
    }
}
