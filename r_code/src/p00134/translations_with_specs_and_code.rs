use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line to get the number of inputs
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().expect("Expected a positive integer"),
        _ => return, // Undefined behavior if no input is provided
    };

    // Preconditions check: n must be positive
    if n == 0 {
        return; // Undefined behavior if n is zero or negative
    }

    let mut sum = 0;

    // Read the subsequent n integers
    for _ in 0..n {
        let in_value: i32 = match lines.next() {
            Some(Ok(line)) => line.trim().parse().expect("Expected an integer"),
            _ => return, // Undefined behavior if fewer than n integers are provided
        };
        sum += in_value;
    }

    // Output the average as an integer
    println!("{}", sum / n as i32);
}
