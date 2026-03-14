fn main() {
    use std::io::{self, Write};

    let mut ch = String::new();

    // Read a line of input from stdin
    io::stdin().read_line(&mut ch).expect("Failed to read line");

    // Remove the newline character if it exists
    if let Some('\n') = ch.chars().next_back() {
        ch.pop();
    }

    // Convert lowercase letters to uppercase
    for c in ch.chars_mut() {
        if ('a'..='z').contains(&c) {
            c = (c as u8 - 32) as char; // Emulate the C behavior of subtracting 32
        }
    }

    // Output the modified string
    println!("{}", ch);
}
