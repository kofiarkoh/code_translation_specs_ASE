use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input to remove any trailing newline characters
    let input = input.trim();

    // Check the length of the input string
    if input.len() > 99 {
        // Behavior is not defined if the input exceeds 99 characters
        // We will simply return 0 in this case as a placeholder
        // since the specification does not define behavior for this case.
        println!("0");
        return;
    }

    // Create a HashMap to count the frequency of each character
    let mut frequency: HashMap<char, usize> = HashMap::new();

    // Process each character in the input string
    for ch in input.chars() {
        // Only consider ASCII characters as per the specification
        if ch.is_ascii() {
            *frequency.entry(ch).or_insert(0) += 1;
        }
    }

    // Calculate the result based on the frequencies
    // The specification does not define the exact logic for this calculation,
    // so we will sum the frequencies as a reasonable interpretation.
    let result: usize = frequency.values().sum();

    // Output the result, ensuring it is non-negative
    println!("{}", result);
}
