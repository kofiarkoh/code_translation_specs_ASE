use std::io::{self, Write};

fn main() {
    // Read a single integer value from standard input
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure prompt is shown before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as an integer
    let input_value: i32 = input.trim().parse().expect("Input must be a valid integer");

    // Calculate one-third of the input value
    let one_third = input_value as f64 / 3.0;

    // Calculate the cube of one-third
    let result = one_third.powi(3);

    // Print the result formatted to twelve decimal places
    println!("{:.12}", result);
}
