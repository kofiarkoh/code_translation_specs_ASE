use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Trim the input to remove any trailing newline characters
    let input = input.trim();
    
    // Ensure the input does not exceed 10 characters
    if input.len() > 10 {
        return;
    }

    let mut output = String::new();

    for c in input.chars() {
        if c == 'B' {
            output.pop(); // Remove the last character from output
        } else {
            output.push(c); // Add character to output
        }
    }

    // Output the modified string
    print!("{}", output);
    io::stdout().flush().expect("Failed to flush stdout");
}
