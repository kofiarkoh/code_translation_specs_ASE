use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let mut case_number = 0;

    loop {
        // Read input from stdin
        input.clear(); // Clear the previous input
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse the input as an integer
        let x: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input and continue the loop
        };

        // Check for the sentinel value
        if x == 0 {
            break; // Exit the loop if the sentinel value is encountered
        }

        // Increment the case number and print the formatted output
        case_number += 1;
        println!("Case {}: {}", case_number, x);
    }

    // Return 0 upon successful completion
    std::process::exit(0);
}
