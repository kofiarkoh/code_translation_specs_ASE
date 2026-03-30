use std::io::{self, Write};

fn main() {
    let mut s = String::new();
    let mut b = [0; 256]; // Array to count occurrences of each character
    let mut a = 0; // Result variable
    let mut c = 0; // Count of consecutive characters
    let mut l = 0; // Length of the input string

    // Read input string
    io::stdout().flush().unwrap(); // Ensure prompt is printed before input
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim(); // Remove trailing newline

    // Count occurrences of each character
    l = s.len() as i32; // Length of the string
    for ch in s.chars() {
        b[ch as usize] += 1;
    }

    // Process the character counts
    while l > 0 {
        for i in 0..256 {
            if b[i] > 0 {
                a += 1; // Increment result for each unique character
                l -= 1; // Decrease length
                c += 1; // Increase consecutive count
                b[i] -= 1; // Decrease count of this character
            } else {
                if c > 3 {
                    a -= c - 3; // Adjust result if more than 3 consecutive
                }
                c = 0; // Reset consecutive count
            }
        }
    }

    // Print the result
    println!("{}", a);
}
