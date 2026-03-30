use std::io::{self, Write};

fn main() {
    let mut case_number = 0;
    loop {
        // Prompt for input
        print!("Enter an integer (0 to exit): ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            // Handle potential I/O error (though spec does not require error handling)
            eprintln!("Error reading input.");
            continue;
        }

        // Parse the input as an integer
        match input.trim().parse::<i32>() {
            Ok(x) => {
                if x == 0 {
                    break; // Terminate on zero input
                }
                case_number += 1; // Increment case number for each valid input
                println!("Case {}: {}", case_number, x); // Output the case number and value
            }
            Err(_) => {
                // Spec states that invalid inputs lead to undefined behavior, so we do nothing here
                // In a real-world scenario, we might want to handle this case
            }
        }
    }
}
