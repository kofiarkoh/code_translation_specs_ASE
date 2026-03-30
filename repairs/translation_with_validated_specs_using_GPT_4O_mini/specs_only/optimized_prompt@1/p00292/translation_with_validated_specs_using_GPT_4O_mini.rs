use std::io::{self, BufRead};

fn main() {
    // Read the first line of input, which indicates the number of pairs
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Parse the number of pairs
    let num_pairs: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0), // Handle parsing errors by defaulting to 0
        _ => return, // Exit if there's an error reading the input
    };

    // Process each pair of integers
    for _ in 0..num_pairs {
        // Read the next line containing the pair of integers
        if let Some(Ok(line)) = lines.next() {
            let mut nums = line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok()); // Parse integers, ignoring any parsing errors

            // Get the first and second integers
            if let (Some(x), Some(y)) = (nums.next(), nums.next()) {
                // Compute the derived value using the specified formula
                let result = (x - 1) % y + 1; // This will always yield a positive integer as per the specification
                println!("{}", result);
            }
        }
    }
}
