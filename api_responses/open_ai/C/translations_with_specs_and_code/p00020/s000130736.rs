use std::io::{self, Write};

fn main() {
    // Create a buffer to hold the input string, with a maximum length of 80 characters.
    let mut input = String::new();

    // Read a line from standard input. We use `read_line` to ensure we handle input safely.
    // Note: `read_line` will include the newline character, so we need to trim it later.
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Error reading input: {}", e);
        return;
    }

    // Trim the input to remove any trailing newline characters.
    let input = input.trim();

    // Check if the input exceeds the maximum length of 80 characters.
    if input.len() > 80 {
        // Behavior is undefined if the input exceeds the maximum length, as per the spec.
        // We will simply return without doing anything.
        return;
    }

    // Create a mutable String to hold the output.
    let mut output = String::with_capacity(input.len());

    // Convert lowercase letters to uppercase.
    for ch in input.chars() {
        if ch.is_ascii_lowercase() {
            output.push(ch.to_ascii_uppercase());
        } else {
            output.push(ch);
        }
    }

    // Print the modified string to standard output.
    // The output will automatically include a newline at the end.
    println!("{}", output);
}
