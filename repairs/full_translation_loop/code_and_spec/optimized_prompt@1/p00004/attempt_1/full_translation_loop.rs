use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut values = line.split_whitespace()
            .filter_map(|s| s.parse::<f64>().ok());

        // Collect six floating-point values
        let a = match values.next() {
            Some(v) if v != 0.0 => v, // Ensure the first value is non-zero
            _ => continue, // Skip if the first value is zero or not present
        };
        let b = values.next().unwrap_or(0.0);
        let c = values.next().unwrap_or(0.0);
        let d = values.next().unwrap_or(0.0);
        let e = values.next().unwrap_or(0.0);
        let f = values.next().unwrap_or(0.0);

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
