use std::io::{self, BufRead};

fn main() {
    // Read lines from standard input until EOF
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Handle each line of input
        match line {
            Ok(input) => {
                let output = process_line(&input);
                println!("{}", output);
            }
            Err(_) => {
                // If reading the line fails, we ignore it as per the specification
            }
        }
    }
}

// Function to process a single line of input
fn process_line(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable(); // Use peekable to look ahead

    while let Some(c) = chars.next() {
        if c == '@' {
            // If we encounter '@', we expect a digit and a character
            if let Some(digit_char) = chars.next() {
                if let Some(repeat_char) = chars.next() {
                    // Check if the character is a digit
                    if digit_char.is_digit(10) {
                        // Convert the digit character to a number
                        let repeat_count = digit_char.to_digit(10).unwrap() as usize;
                        // Repeat the character and append to output
                        output.push_str(&repeat_char.to_string().repeat(repeat_count));
                    }
                    // If the character is not a digit, we do nothing as per the spec
                }
            }
        } else {
            // If it's not '@', just add the character to output
            output.push(c);
        }
    }

    output
}
