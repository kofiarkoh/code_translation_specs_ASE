use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // The first number indicates the length of the sequence
        if numbers.is_empty() || numbers[0] == 0 {
            break; // Termination condition
        }

        let length = numbers[0] as usize;

        // Ensure we only process valid sequences
        if numbers.len() - 1 != length {
            continue; // Invalid sequence length, skip processing
        }

        let mut count_zeros = 0;

        for &num in &numbers[1..] {
            if num == 0 {
                count_zeros += 1; // Count zeros
            }
        }

        // Output the count of zeros in the sequence
        println!("{}", count_zeros);
    }
}
