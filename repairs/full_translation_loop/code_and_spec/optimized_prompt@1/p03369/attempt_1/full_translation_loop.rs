use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim whitespace and ensure the input is exactly 3 characters
    let input = input.trim();
    if input.len() != 3 {
        // Undefined behavior as per specification; we could panic or handle it differently
        panic!("Input must consist of exactly three characters.");
    }

    // Count occurrences of 'o'
    let count_o = input.chars().filter(|&c| c == 'o').count();

    // Calculate the output value
    let output_value = 700 + (100 * count_o);

    // Print the result
    println!("{}", output_value);
}
