use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Split the input into five integers
    let numbers: Vec<i32> = input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have exactly five integers
    if numbers.len() != 5 {
        // Behavior is not defined for incorrect input, so we do nothing
        return;
    }

    // Check for the first zero and print the corresponding position
    for (index, &number) in numbers.iter().enumerate() {
        if number == 0 {
            // Output the position (1-based index)
            println!("{}", index + 1);
            return; // Exit after finding the first zero
        }
    }
}
