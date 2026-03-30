use std::io;

fn main() {
    let mut case_number = 1; // Initialize case number

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Attempt to parse the input as an integer
        match input.trim().parse::<i32>() {
            Ok(0) => break, // Exit loop if input is zero
            Ok(value) => {
                // Output case number and value for non-zero input
                println!("Case {}: {}", case_number, value);
                case_number += 1; // Increment case number
            }
            Err(_) => {
                // Undefined behavior for invalid input as per specification
                // No handling of invalid inputs is implemented
            }
        }
    }
}
