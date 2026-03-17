fn main() {
    use std::io::{self, Write};

    // Read input from stdin
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input to remove any trailing newline characters
    let input = input.trim();

    // Check precondition: input must consist of exactly three characters
    if input.len() != 3 {
        panic!("Input must consist of exactly three characters.");
    }

    // Count occurrences of 'o'
    let count_o = input.chars().filter(|&c| c == 'o').count();

    // Calculate the output value
    let output_value = 700 + (100 * count_o);

    // Print the output value
    println!("{}", output_value);
}
