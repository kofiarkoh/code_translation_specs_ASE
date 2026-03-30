use std::io;

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    if io::stdin().read_line(&mut input).is_ok() {
        // Attempt to parse the input as an integer
        match input.trim().parse::<i32>() {
            Ok(m) => {
                // Calculate the difference from 48 and print the result
                println!("{}", 48 - m);
            }
            Err(_) => {
                // Handle the case where input is not a valid integer
                eprintln!("Invalid input. Please enter a valid integer.");
            }
        }
    } else {
        // Handle the case where reading from stdin fails
        eprintln!("Failed to read input.");
    }
}
