use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let input = line.expect("Failed to read line");
        let output = process_string(&input);
        println!("{}", output);
    }
}

fn process_string(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '@' {
            // Check the next character
            if let Some(&next_char) = chars.peek() {
                if next_char.is_digit(10) {
                    // If the next character is a digit, include it and any subsequent characters
                    output.push(c);
                    output.push(next_char);
                    chars.next(); // consume the digit
                    output.extend(chars.by_ref()); // include the rest of the string
                    break; // exit the loop as we've processed the rest
                } else {
                    // If '@' is not followed by a digit, just add '@' to output
                    output.push(c);
                }
            } else {
                // If '@' is the last character, just add it
                output.push(c);
            }
        } else {
            // Regular character, just add to output
            output.push(c);
        }
    }

    output
}
