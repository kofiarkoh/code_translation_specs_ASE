use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    let mut string = String::new();

    // Read the integer n
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please enter a valid number");

    // Read the string
    io::stdin().read_line(&mut string).expect("Failed to read line");
    let string = string.trim(); // Trim to remove any trailing newline

    let mut count = 0;
    let mut i = 0;

    // Loop through the string, checking for "ABC"
    while i < n.saturating_sub(2) { // Use saturating_sub to avoid underflow
        if &string[i..i + 3] == "ABC" {
            count += 1;
            i += 3; // Move past the found "ABC"
        } else {
            i += 1; // Move to the next character
        }
    }

    // Print the count
    print!("{}", count);
}
