use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line to get the number of boolean characters
    let mut lines = reader.lines();
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or_else(|_| {
            eprintln!("Invalid input for the number of characters.");
            std::process::exit(1);
        }),
        _ => {
            eprintln!("Failed to read the number of characters.");
            std::process::exit(1);
        }
    };

    // Check precondition: n must be >= 2
    if n < 2 {
        eprintln!("The number of characters must be at least 2.");
        std::process::exit(1);
    }

    // Initialize a counter for 'T' characters
    let mut true_count = 0;

    // Read the boolean characters and validate them
    for _ in 0..n {
        match lines.next() {
            Some(Ok(line)) => {
                let ch = line.trim();
                if ch == "T" {
                    true_count += 1;
                } else if ch == "F" {
                    // Do nothing, just continue
                } else {
                    eprintln!("Invalid character encountered: {}", ch);
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Failed to read a boolean character.");
                std::process::exit(1);
            }
        }
    }

    // Determine the result based on the count of 'T' characters
    let result = if true_count > n / 2 { 'T' } else { 'F' };

    // Output the result
    println!("{}", result);
}
