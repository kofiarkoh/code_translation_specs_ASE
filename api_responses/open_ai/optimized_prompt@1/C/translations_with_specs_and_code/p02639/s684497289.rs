use std::io::{self, BufRead};

fn main() {
    // Read five integers from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap()) // Assumes valid input as per specification
        .collect();

    // Check for the first zero and print the corresponding position
    for (i, &num) in numbers.iter().enumerate() {
        if num == 0 {
            // Output the position (1-based index)
            println!("{}", i + 1);
            return; // Exit after finding the first zero
        }
    }
    // No output if no zero is found, as per specification
}
