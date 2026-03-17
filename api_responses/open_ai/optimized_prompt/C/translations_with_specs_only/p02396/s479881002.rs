use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut case_number = 1;

    for line in reader.lines() {
        // Read each line from standard input
        if let Ok(input) = line {
            // Attempt to parse the input as an integer
            if let Ok(value) = input.trim().parse::<i32>() {
                // Check for the termination condition (zero)
                if value == 0 {
                    break; // Terminate on zero
                }
                // Output the case number and the integer value
                println!("Case {}: {}", case_number, value);
                case_number += 1; // Increment case number for the next non-zero integer
            }
            // The specification states that invalid input is not handled,
            // so we do not take any action on parsing errors.
        }
    }
}
