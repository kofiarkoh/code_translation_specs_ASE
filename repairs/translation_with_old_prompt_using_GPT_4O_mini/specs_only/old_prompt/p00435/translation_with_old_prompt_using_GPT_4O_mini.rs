use std::io::{self, Write};

fn main() {
    // Read input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove the newline character at the end of the input
    let input = input.trim();

    // Create a mutable vector to hold the transformed characters
    let mut transformed: Vec<char> = Vec::with_capacity(input.len());

    // Transform each character according to the specified rules
    for c in input.chars() {
        let transformed_char = if c.is_ascii_uppercase() {
            // Shift uppercase letters backward by 3
            let shifted = c as u8 as i32 - 3;
            // Wrap around if it goes below 'A'
            if shifted < 'A' as i32 {
                (shifted + 26) as u8 as char
            } else {
                shifted as u8 as char
            }
        } else {
            // Shift other characters forward by 23
            let shifted = c as u8 as i32 + 23;
            // Wrap around if it goes above 'z'
            if shifted > 'z' as i32 {
                (shifted - 26) as u8 as char
            } else {
                shifted as u8 as char
            }
        };
        transformed.push(transformed_char);
    }

    // Output the transformed string
    println!("{}", transformed.iter().collect::<String>());

    // Return 0 to indicate successful execution
    std::process::exit(0);
}
