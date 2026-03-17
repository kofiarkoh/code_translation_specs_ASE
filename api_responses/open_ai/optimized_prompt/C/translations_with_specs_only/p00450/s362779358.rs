use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check if the first number is zero, which indicates termination
        if numbers.is_empty() || numbers[0] == 0 {
            break;
        }

        // The first number indicates how many subsequent integers to process
        let count = numbers[0] as usize;

        // Ensure we only process the specified number of integers
        if numbers.len() < count + 1 {
            continue; // Not enough numbers, skip this sequence
        }

        // Process the sequence: replace certain values and count zeros
        let mut zero_count = 0;
        for i in 1..=count {
            // Example transformation: replace negative numbers with zero
            if numbers[i] < 0 {
                numbers[i] = 0;
            }
            // Count zeros in the modified sequence
            if numbers[i] == 0 {
                zero_count += 1;
            }
        }

        // Output the count of zeros for the current sequence
        println!("{}", zero_count);
    }
}
