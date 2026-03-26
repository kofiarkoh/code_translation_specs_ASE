use std::io::{self, Write};

fn main() {
    // Prompt the user for input
    print!("Please enter an integer: ");
    // Flush to ensure the prompt is displayed before reading input
    io::stdout().flush().unwrap();

    // Read input from the user
    let mut input = String::new();
    if let Err(_) = io::stdin().read_line(&mut input) {
        // If reading input fails, we do not handle it gracefully as per specification
        return;
    }

    // Attempt to parse the input as an integer
    let input_value: i32 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            // If parsing fails, behavior is undefined; we do not handle it
            return;
        }
    };

    // Perform the calculation (48 - input)
    let result = 48 - input_value;

    // Output the result
    println!("{}", result);

    // The program will return 0 upon successful completion
}
