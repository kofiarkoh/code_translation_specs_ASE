use std::io::{self, Write};

fn main() {
    // Read the number of boolean characters to process
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the number of characters
    let n: usize = match input.trim().parse() {
        Ok(num) if num >= 2 => num,
        _ => {
            eprintln!("Invalid input for the number of characters.");
            std::process::exit(1);
        }
    };

    // Read the boolean characters
    let mut boolean_chars = String::new();
    io::stdin().read_line(&mut boolean_chars).expect("Failed to read line");

    // Check if the number of characters matches the expected count
    if boolean_chars.trim().len() != n {
        eprintln!("The number of boolean characters does not match the specified count.");
        std::process::exit(1);
    }

    // Validate that all characters are either 'T' or 'F'
    for c in boolean_chars.trim().chars() {
        if c != 'T' && c != 'F' {
            eprintln!("Invalid character found: {}", c);
            std::process::exit(1);
        }
    }

    // Evaluate the logical operation (AND operation)
    let result = boolean_chars.trim().chars().all(|c| c == 'T');

    // Output the result as 'T' for true and 'F' for false
    let output = if result { 'T' } else { 'F' };
    print!("{}", output);
    io::stdout().flush().expect("Failed to flush output");
}
