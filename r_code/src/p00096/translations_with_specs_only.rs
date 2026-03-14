use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(input) => {
                // Attempt to parse the input as a non-negative integer
                if let Ok(value) = input.trim().parse::<u32>() {
                    let cumulative_metric = compute_cumulative_metric(value);
                    println!("{}", cumulative_metric);
                }
                // If parsing fails, the behavior is undefined as per the specification
                // No error handling is implemented, so we simply ignore invalid input
            }
            Err(_) => {
                // If there's an error reading the line, we gracefully exit
                break;
            }
        }
    }
}

// Function to compute the cumulative metric based on the input value
fn compute_cumulative_metric(value: u32) -> u32 {
    let mut cumulative = 0;

    // Iterate through all combinations of non-negative integers that do not exceed the input value
    for i in 0..=value {
        for j in 0..=value {
            // Example metric calculation: sum of the two integers
            cumulative += i + j;
        }
    }

    cumulative
}
