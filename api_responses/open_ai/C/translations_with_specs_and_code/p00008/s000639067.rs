use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        // Read each line and parse it as an integer
        if let Ok(input) = line {
            if let Ok(n) = input.trim().parse::<i32>() {
                // Count combinations of four digits that sum to n
                let count = count_combinations(n);
                // Print the result for the current input
                println!("{}", count);
            }
        }
    }
}

fn count_combinations(n: i32) -> i32 {
    // Initialize count of combinations
    let mut count = 0;

    // Iterate through all combinations of four digits (0-9)
    for a in 0..=9 {
        for b in 0..=9 {
            for c in 0..=9 {
                for d in 0..=9 {
                    // Check if the sum of the digits equals n
                    if a + b + c + d == n {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}
