use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok());

        // Read six integers from the input
        let a = match nums.next() { Some(n) => n, None => break };
        let b = match nums.next() { Some(n) => n, None => break };
        let c = match nums.next() { Some(n) => n, None => break };
        let d = match nums.next() { Some(n) => n, None => break };
        let e = match nums.next() { Some(n) => n, None => break };
        let f = match nums.next() { Some(n) => n, None => break };

        // Calculate x and y using f64 for floating-point division
        let x = (c as f64 * e as f64 - b as f64 * f as f64) / (a as f64 * e as f64 - b as f64 * d as f64);
        let y = (c as f64 * d as f64 - a as f64 * f as f64) / (b as f64 * d as f64 - a as f64 * e as f64);

        // Adjust x and y based on the specified conditions
        let x = if -0.0005 < x && x <= 0.0 { 0.0 } else { x };
        let y = if -0.0005 < y && y <= 0.0 { 0.0 } else { y };

        // Print the results with the specified formatting
        println!("{:.3} {:.3}", x, y);
    }
}
