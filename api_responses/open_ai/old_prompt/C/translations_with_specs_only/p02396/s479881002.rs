use std::io::{self, Write};

fn main() -> i32 {
    let mut case_number = 1; // Initialize case number
    loop {
        let mut input = String::new(); // Buffer for user input
        print!("Enter an integer (0 to exit): "); // Prompt for input
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before input

        // Read input from standard input
        if io::stdin().read_line(&mut input).is_err() {
            // If reading fails, we exit with an error code
            return 1; // Indicate an error occurred
        }

        // Attempt to parse the input as an integer
        match input.trim().parse::<i32>() {
            Ok(num) => {
                if num == 0 {
                    break; // Exit loop if sentinel value (0) is entered
                }
                // Print the formatted output for valid input
                println!("Case {}: {}", case_number, num);
                case_number += 1; // Increment case number for the next valid input
            }
            Err(_) => {
                // If parsing fails, we do not handle it as per the specification
                // The program may behave unexpectedly, as stated in the spec
                // No output or error message is provided for invalid input
            }
        }
    }
    0 // Return 0 upon successful completion
}
