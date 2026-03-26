use std::io::{self, Write};

fn main() {
    // We will perform exactly 7 iterations as specified.
    for _ in 0..7 {
        // Declare variables to hold the input integers.
        let mut a = String::new();
        let mut b = String::new();

        // Read the first integer from standard input.
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before input
        io::stdin().read_line(&mut a).expect("Failed to read line");
        
        // Read the second integer from standard input.
        io::stdin().read_line(&mut b).expect("Failed to read line");

        // Attempt to parse the input strings into integers.
        // If parsing fails, we will handle the error gracefully by skipping the iteration.
        let a: i32 = match a.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Invalid input for a, skipping this iteration.");
                continue; // Skip to the next iteration on error
            }
        };
        
        let b: i32 = match b.trim().parse() {
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
