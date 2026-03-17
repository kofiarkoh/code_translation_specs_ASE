fn main() {
    use std::io::{self, Read};

    // Read the input string from standard input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Ensure the input string does not exceed 999 characters
    if input.len() > 999 {
        panic!("Input string exceeds the maximum length of 999 characters");
    }

    // Create a mutable String to hold the modified output
    let mut output = String::with_capacity(input.len());

    // Process each character in the input string
    for c in input.chars() {
        let modified_char = if c.is_ascii_uppercase() {
            // Shift uppercase letters (A-Z) backward by 3 positions
            // ASCII 'A' is 65, 'Z' is 90
            // Ensure wraparound for characters less than 'A'
            let shifted = c as u8 - 3;
            if shifted < b'A' {
                (shifted + 26) as char // Wrap around to the end of uppercase letters
            } else {
                shifted as char
            }
        } else {
            // Shift all other characters forward by 23 positions
            // Ensure wraparound for characters that exceed the ASCII range
            let shifted = c as u8 + 23;
            if shifted > 127 {
                (shifted - 128) as char // Wrap around to the start of ASCII
            } else {
                shifted as char
            }
        };
        output.push(modified_char);
    }

    // Output the modified string
    println!("{}", output);
}
