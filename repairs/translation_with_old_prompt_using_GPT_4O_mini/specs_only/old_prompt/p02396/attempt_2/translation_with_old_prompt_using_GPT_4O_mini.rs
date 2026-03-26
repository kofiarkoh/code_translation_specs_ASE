use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut case_number = 1; // Initialize case number
    loop {
        let mut input = String::new(); // Buffer for user input
        print!("Enter an integer (0 to exit): "); // Prompt for input
        io::stdout().flush()?; // Ensure prompt is displayed immediately

        // Read input from standard input
        if io::stdin().read_line(&mut input).is_err() {
            // If reading fails, we exit with an error
            return Err("Failed to read input".into()); // Return an error
        }

        // Parse the input as an integer
        let input_value: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input and continue the loop
        };

        // Check for the sentinel value
        if input_value == 0 {
            break; // Exit the loop if sentinel value is entered
        }

        // Output the formatted case number and input value
        println!("Case {}: {}", case_number, input_value);
        case_number += 1; // Increment case number for the next valid input
    }

    Ok(()) // Return Ok upon successful completion
}
