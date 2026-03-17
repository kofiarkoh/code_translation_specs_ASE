use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Trim the input to remove any surrounding whitespace
    let input = input.trim();

    // Check the length constraint (not exceeding 100 characters)
    if input.len() > 100 {
        // Undefined behavior as per specification; we do nothing
        return;
    }

    // Create a HashMap to count the frequency of each character
    let mut frequency: HashMap<char, usize> = HashMap::new();

    // Count the frequency of each character in the input string
    for ch in input.chars() {
        // Only consider ASCII characters (0-127)
        if ch.is_ascii() {
            *frequency.entry(ch).or_insert(0) += 1;
        } else {
            // Undefined behavior as per specification; we do nothing
            return;
        }
    }

    // Calculate the result based on unique characters and their frequencies
    let unique_count = frequency.len();
    let result = unique_count as i32; // Start with the count of unique characters

    // Adjust the result based on the frequency of characters
    for &count in frequency.values() {
        // Example adjustment: if a character appears more than once, we subtract 1
        if count > 1 {
            result -= 1;
        }
    }

    // Output the computed result
    println!("{}", result);
}
