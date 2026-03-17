fn main() {
    use std::io::{self, Write};

    let mut input = String::new();
    
    // Read the length of the string
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for length");

    // Preconditions check
    if n < 3 {
        // Not explicitly handled as per the specification
        return;
    }

    input.clear(); // Clear the input buffer for the next read

    // Read the string
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let string = input.trim();

    // Ensure the string is at least n characters long
    if string.len() < n {
        // Behavior is not defined as per the specification
        return;
    }

    let mut count = 0;
    let mut i = 0;

    // Count non-overlapping occurrences of "ABC"
    while i <= n - 3 {
        if &string[i..i + 3] == "ABC" {
            count += 1;
            i += 3; // Move past the found substring
        } else {
            i += 1; // Move to the next character
        }
    }

    // Output the count
    println!("{}", count);
}
