use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    let mut string = String::new();

    // Read the length of the string
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before input
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please enter a valid number");

    // Read the string input
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before input
    io::stdin().read_line(&mut string).expect("Failed to read line");
    let string = string.trim(); // Remove any trailing newline

    // Check precondition
    if n < 3 {
        println!("0");
        return;
    }

    // Ensure the string is at least n characters long
    if string.len() < n {
        // Behavior is not defined as per the spec, but we can handle it gracefully
        println!("0");
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
