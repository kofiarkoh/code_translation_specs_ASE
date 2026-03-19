use std::io::{self, Write};

fn main() {
    // Read the number of characters to evaluate
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        // If reading fails, terminate with a non-zero exit status
        std::process::exit(1);
    }

    let count: usize = match input.trim().parse() {
        Ok(n) if n >= 0 => n, // Ensure non-negative
        _ => {
            std::process::exit(1); // Terminate on invalid input
        }
    };

    // Read the string of characters
    let mut char_input = String::new();
    if io::stdin().read_line(&mut char_input).is_err() {
        // If reading fails, terminate with a non-zero exit status
        std::process::exit(1);
    }

    // Ensure the string is at least as long as the count specified
    let char_input = char_input.trim();
    if char_input.len() < count {
        std::process::exit(1); // Terminate if string is too short
    }

    // Count 'R' characters in the specified range
    let mut r_count = 0;
    for c in char_input.chars().take(count) {
        if c == 'R' {
            r_count += 1; // Update count of 'R' characters
        }
    }

    // Determine if 'R' count exceeds half of the total characters evaluated
    let result = if r_count > count / 2 {
        "Yes"
    } else {
        "No"
    };

    // Output the result
    println!("{}", result);
}
