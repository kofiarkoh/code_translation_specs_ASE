use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line to get `n`
    let n: i32 = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => return, // Handle invalid input gracefully
    };

    // If n is zero or negative, output "IMPOSSIBLE"
    if n <= 0 {
        println!("IMPOSSIBLE");
        return;
    }

    let mut p = 0; // Counter for conditions
    let mut possible = true; // Flag to track if the outcome is possible

    // Read pairs of (a, b) until EOF
    for line in lines {
        if let Ok(line) = line {
            let nums: Vec<i32> = line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if nums.len() < 2 {
                break; // Exit if we don't have enough numbers
            }

            let a = nums[0];
            let b = nums[1];

            // Evaluate the conditions
            if p != 0 && n != 0 {
                // If p is not zero and n is not zero, set possible to false
                possible = false;
                break; // No need to continue if we already found it impossible
            } else {
                // Increment p
                p += 1;
            }

            // Update p based on conditions
            p = if a != 0 {
                if b != n {
                    0
                } else {
                    a
                }
            } else {
                b
            };
        }
    }

    // Output the result based on the possible flag
    if possible {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
