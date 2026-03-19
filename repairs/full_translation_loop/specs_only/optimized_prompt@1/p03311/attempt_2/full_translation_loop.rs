use std::io::{self, BufRead};

fn main() {
    // Read the number of elements from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line to get the number of elements
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.parse().unwrap_or(0), // Default to 0 if parsing fails
        _ => return, // Exit if there's an error reading the line
    };

    // Initialize a vector to hold the integers
    let mut values = Vec::with_capacity(n);

    // Read the subsequent lines to get the integers
    for _ in 0..n {
        match lines.next() {
            Some(Ok(line)) => {
                if let Ok(value) = line.parse::<i64>() {
                    // Apply transformation: here we assume the transformation is identity
                    values.push(value);
                }
            }
            _ => return, // Exit if there's an error reading the line
        }
    }

    // Compute the minimum value from the processed integers
    // The output must be a non-negative integer, so we filter out negatives
    let min_value = values.iter().cloned().filter(|&x| x >= 0).min().unwrap_or(0);

    // Output the computed minimum value
    println!("{}", min_value);
}
