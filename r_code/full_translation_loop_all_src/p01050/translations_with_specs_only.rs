use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Trim the input to remove any trailing newline characters
    let input = input.trim();

    // Check the length of the input string
    if input.len() > 99 {
        // Behavior is not defined for input exceeding 99 characters
        return;
    }

    // Create a HashMap to count the frequency of each character
    let mut frequency: HashMap<char, usize> = HashMap::new();

    // Process each character in the input string
    for c in input.chars() {
        // Count only ASCII characters
        if c.is_ascii() {
            *frequency.entry(c).or_insert(0) += 1;
        }
    }

    // Calculate the result based on the frequencies
    let result: usize = frequency.values().sum();

    // Output the result as a non-negative integer
    println!("{}", result);
}
