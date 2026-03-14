use std::io::{self, BufRead};

fn main() {
    // Create an array to count occurrences of integers from 1 to 100
    let mut counts = [0; 101]; // Index 0 will be unused

    // Read integers from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        if let Ok(input) = line {
            if let Ok(num) = input.trim().parse::<usize>() {
                // Only count numbers in the range 1 to 100
                if num >= 1 && num <= 100 {
                    counts[num] += 1;
                }
            }
        }
    }

    // Find the maximum frequency and the corresponding integers
    let max_frequency = *counts.iter().max().unwrap_or(&0);
    
    // Output all integers that have the maximum frequency
    if max_frequency > 0 {
        for (i, &count) in counts.iter().enumerate() {
            if count == max_frequency {
                println!("{}", i);
            }
        }
    }
}
