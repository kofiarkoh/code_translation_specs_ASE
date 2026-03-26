use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut numbers = line.split_whitespace()
            .filter_map(|s| s.parse::<f64>().ok());

        // Read six double values
        let a = match numbers.next() {
            Some(value) => value,
            None => continue, // Skip if not enough values
        };
        let b = match numbers.next() {
            Some(value) => value,
            None => continue,
        };
        let c = match numbers.next() {
            Some(value) => value,
            None => continue,
        };
        let d = match numbers.next() {
            Some(value) => value,
            None => continue,
        };
        let e = match numbers.next() {
            Some(value) => value,
            None => continue,
        };
        let f = match numbers.next() {
            Some(value) => value,
            None => continue,
        };

        // Check for division by zero
        if a == 0.0 {
            eprintln!("Error: Division by zero (a is zero)");
            continue; // Skip this iteration
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
