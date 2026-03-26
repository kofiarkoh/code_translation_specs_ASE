use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read a single line of input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into parts and parse them as integers
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();
    
    // Ensure exactly five integers are provided
    if numbers.len() != 5 {
        eprintln!("Error: Exactly five integers are required.");
        return;
    }

    // Check each number for zero and print the corresponding index
    for (index, &number) in numbers.iter().enumerate() {
        if number == 0 {
            // Output the position (1-based index)
            print!("{}", index + 1);
            return; // Exit after the first zero is found
        }
    }
}
