use std::io::{self, BufRead};

fn main() {
    // Read from standard input line by line
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // Handle each line of input
        if let Ok(input) = line {
            process_input(&input);
        }
    }
}

fn process_input(input: &str) {
    // Initialize an output string
    let mut output = String::new();
    let mut chars = input.chars().peekable(); // Use peekable to look ahead

    while let Some(c) = chars.next() {
        if c == '@' {
            // Check if the next character is a digit
            if let Some(&next_char) = chars.peek() {
                if next_char.is_digit(10) {
                    // Consume the digit
                    let digit = chars.next().unwrap(); // Safe to unwrap since we checked
                    let count: usize = digit.to_digit(10).unwrap() as usize; // Convert char to digit

                    // Check if the next character is valid
                    if let Some(&repeat_char) = chars.peek() {
                        // Consume the character to repeat
                        chars.next(); // Safe to consume since we checked
                        // Append the repeated character to output
                        output.push_str(&repeat_char.to_string().repeat(count));
                    } else {
                        // If no character to repeat, just output '@' and the digit
                        output.push('@');
                        output.push(digit);
                    }
                } else {
                    // If the character after '@' is not a digit, output as-is
                    output.push(c);
                }
            } else {
                // If '@' is the last character, output as-is
                output.push(c);
            }
        } else {
            // For all other characters, just append to output
            output.push(c);
        }
    }

    // Print the processed output followed by a newline
    println!("{}", output);
}
