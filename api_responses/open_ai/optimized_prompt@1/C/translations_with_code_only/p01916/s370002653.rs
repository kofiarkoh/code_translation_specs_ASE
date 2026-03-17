use std::io::{self, Write};

fn main() {
    let mut str = String::new();
    let mut a = [0; 30]; // Array to track occurrences of characters
    let mut sum = 0;

    // Read input string
    io::stdin().read_line(&mut str).expect("Failed to read line");
    let str = str.trim(); // Remove any trailing newline or whitespace

    // Toggle the occurrence of each character
    for c in str.chars() {
        if c.is_ascii_lowercase() {
            let index = (c as usize) - ('a' as usize);
            a[index] = 1 - a[index]; // Toggle between 0 and 1
        }
    }

    // Sum the unique characters
    for &count in &a {
        sum += count;
    }

    // Output the result, dividing by 2 as in the original C code
    println!("{}", sum / 2);
}
