use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut case_number = 1;

    for line in handle.lines() {
        // Read each line from standard input
        if let Ok(input) = line {
            // Parse the input as an integer
            if let Ok(value) = input.trim().parse::<i32>() {
                // Check for termination condition (zero)
                if value == 0 {
                    break; // Terminate on zero
                }
                // Output the formatted string for non-zero integers
                println!("Case {}: {}", case_number, value);
                case_number += 1; // Increment case number for each non-zero integer
            }
            // The specification assumes valid integers, so no error handling is implemented
        }
    }
}
