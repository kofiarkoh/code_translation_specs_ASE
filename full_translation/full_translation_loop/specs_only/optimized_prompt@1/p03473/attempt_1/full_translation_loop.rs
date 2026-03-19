use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Attempt to parse the input as an integer
    match input.trim().parse::<i32>() {
        Ok(value) => {
            // Compute the difference from 48
            let result = 48 - value;
            // Output the result followed by a newline
            println!("{}", result);
        }
        Err(_) => {
            // If the input is not a valid integer, behavior is not defined
            // No output is specified for invalid input, so we do nothing
        }
    }
}
