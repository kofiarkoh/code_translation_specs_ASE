fn main() {
    use std::io::{self, Write};

    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input to remove any trailing newline characters
    let input = input.trim();

    // Check the length constraint
    if input.len() > 20 {
        // Behavior is not defined if the input exceeds 20 characters
        // We will simply return without doing anything
        return;
    }

    // Reverse the string
    let reversed: String = input.chars().rev().collect();

    // Output the reversed string
    println!("{}", reversed);
}
