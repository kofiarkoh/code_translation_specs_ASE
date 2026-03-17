use std::io::{self, Write};

fn main() {
    // Read input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove the newline character at the end of the input
    let input = input.trim();

    // Create a mutable String to hold the transformed output
    let mut output = String::with_capacity(input.len());

    // Transform each character according to the specified rules
    for c in input.chars() {
        let transformed_char = match c {
            'A'..='Z' => {
                // Shift uppercase letters backward by 3
                let shifted = c as u8 as i32 - 3;
                // Wrap around if it goes below 'A'
                if shifted < 'A' as i32 {
                    (shifted + 26) as u8 as char
                } else {
                    shifted as u8 as char
                }
            }
            _ => {
                // Shift other characters forward by 23
                let shifted = c as u8 as i32 + 23;
                // Wrap around if it goes above 127 (ASCII limit)
                if shifted > 127 {
                    (shifted - 128) as u8 as char
                } else {
                    shifted as u8 as char
                }
            }
        };
        output.push(transformed_char);
    }

    // Output the transformed string
    println!("{}", output);

    // Return 0 to indicate successful execution
    std::process::exit(0);
}
