use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from standard input
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input
    if let Err(_) = io::stdin().read_line(&mut input) {
        // Handle potential read error (though not specified in the spec)
        return;
    }

    // Attempt to parse the input as an integer
    match input.trim().parse::<i32>() {
        Ok(n) => {
            // Check the value of n and print the corresponding message
            match n {
                25 => println!("Christmas"),
                24 => println!("Christmas Eve"),
                23 => println!("Christmas Eve Eve"),
                22 => println!("Christmas Eve Eve Eve"),
                _ => {} // No output for values outside 22 to 25
            }
        },
        Err(_) => {
            // Undefined behavior for invalid input as per the spec
            // No output or action taken
        }
    }
}
