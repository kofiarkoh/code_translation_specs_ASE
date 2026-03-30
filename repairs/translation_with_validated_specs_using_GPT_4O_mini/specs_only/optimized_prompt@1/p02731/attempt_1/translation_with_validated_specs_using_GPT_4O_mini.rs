use std::io::{self, Write};

fn main() {
    // Read input from the user
    let mut input = String::new();
    print!("Enter an integer: ");
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before input

    // Read the input line
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as an integer
    let length: i32 = input.trim().parse().expect("Input must be a valid integer");

    // Compute one-third of the input
    let one_third = length as f64 / 3.0;

    // Compute the cube of one-third
    let result = one_third.powi(3);

    // Print the result formatted to twelve decimal places
    println!("{:.12}", result);
}
