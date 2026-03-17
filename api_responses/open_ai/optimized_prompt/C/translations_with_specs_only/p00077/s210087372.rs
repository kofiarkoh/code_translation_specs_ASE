use std::io::{self, BufRead};

fn main() {
    // Define a constant for the maximum length of input strings.
    const MAX_LENGTH: usize = 1024; // Assuming a reasonable max length for demonstration.

    // Read from standard input line by line.
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // Handle each line of input.
        match line {
            Ok(input) => {
                // Process the input string and transform it.
                let transformed = process_input(&input);
                // Print the transformed string followed by a newline.
                println!("{}", transformed);
            }
            Err(_) => {
                // If there's an error reading the line, we ignore it as per the spec.
                continue;
            }
        }
    }
}

// Function to process the input string according to the specification.
fn process_input(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable(); // Use peekable to look ahead for patterns.

    while let Some(c) = chars.next() {
        if c == '@' {
            // If we encounter '@', we expect a digit and a following character.
            if let Some(digit_char) = chars.next() {
                if let Some(repeat_char) = chars.next() {
                    // Convert the digit character to a number.
                    if let Some(repeat_count) = digit_char.to_digit(10) {
                        // Repeat the character as specified.
                        output.push_str(&repeat_char.to_string().repeat(repeat_count as usize));
                    }
                    // If the digit is invalid or not a digit, we do nothing as per the spec.
                }
            }
        } else {
            // If it's not '@', we add the character to the output as-is.
            output.push(c);
        }
    }

    output
}
