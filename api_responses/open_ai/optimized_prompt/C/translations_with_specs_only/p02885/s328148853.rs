use std::io::{self, Write};

fn main() {
    // Read two integer values from standard input
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into two parts and parse them as integers
    let mut numbers = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok()); // Filter out invalid integers

    // Get the two integers, defaulting to 0 if not enough valid integers are provided
    let a = numbers.next().unwrap_or(0);
    let b = numbers.next().unwrap_or(0);

    // Perform the arithmetic operation (assumed to be addition for this example)
    let result = a + b;

    // Ensure the output is non-negative
    let output = if result < 0 { 0 } else { result };

    // Print the result
    println!("{}", output);
}
