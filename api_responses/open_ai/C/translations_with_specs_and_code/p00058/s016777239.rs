use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap is safe here as per the spec; we assume valid input
        let numbers: Vec<f64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok()) // Parse and filter valid doubles
            .collect();

        // Ensure we have exactly 8 numbers
        if numbers.len() == 8 {
            let (x1, y1, x2, y2, x3, y3, x4, y4) = (
                numbers[0], numbers[1], 
                numbers[2], numbers[3], 
                numbers[4], numbers[5], 
                numbers[6], numbers[7]
            );

            // Check for parallelism
            if (y1 - y2) * (x3 - x4) + (x1 - x2) * (y3 - y4) == 0.0 {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
