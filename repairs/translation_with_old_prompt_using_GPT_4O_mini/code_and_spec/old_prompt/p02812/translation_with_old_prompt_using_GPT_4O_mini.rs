use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    let mut string = String::new();

    // Read the integer n
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please enter a valid number");

    // Read the string
    io::stdin().read_line(&mut string).expect("Failed to read line");
    let string = string.trim(); // Remove any trailing newline

    // Ensure the input constraints are met
    if n < 3 || n > 50 || string.len() != n {
        // If n is invalid, we do not output anything as per the spec's undefined behavior on invalid input
        return;
    }

    let mut count = 0;
    let mut i = 0;

    // Count occurrences of "ABC"
    while i <= n - 3 {
        if &string[i..i + 3] == "ABC" {
            count += 1;
            i += 2; // Move to the next character after the found substring
        } else {
            i += 1; // Move to the next character
        }
    }

    // Print the count
    println!("{}", count);
}
