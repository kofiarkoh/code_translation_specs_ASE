use std::io::{self, Write};

fn main() {
    // We will perform exactly 7 iterations as specified.
    for _ in 0..7 {
        // Declare a variable to hold the input line.
        let mut input = String::new();

        // Read a line from standard input.
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before input
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Split the input line into parts and attempt to parse them as integers.
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        // Ensure we have exactly two parts for a and b.
        if parts.len() != 2 {
            eprintln!("Expected two integers, skipping this iteration.");
            continue; // Skip to the next iteration if input is not valid
        }

        // Parse the input strings into integers.
        let a: i32 = match parts[0].parse() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Invalid input for a, skipping this iteration.");
                continue; // Skip to the next iteration on error
            }
        };
        
        let b: i32 = match parts[1].parse() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Invalid input for b, skipping this iteration.");
                continue; // Skip to the next iteration on error
            }
        };

        // Calculate the difference and print the result.
        let difference = a - b;
        println!("{}", difference);
    }
}
