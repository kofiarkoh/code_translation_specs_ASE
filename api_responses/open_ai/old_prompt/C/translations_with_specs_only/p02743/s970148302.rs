use std::io::{self, Write};

fn main() {
    // Declare variables for the three long double inputs
    let mut a: f64 = 0.0; // Using f64 as Rust does not have a long double type
    let mut b: f64 = 0.0;
    let mut c: f64 = 0.0;

    // Read inputs from standard input
    // Note: In Rust, we use `read_line` and `parse` to handle input
    let stdin = io::stdin();
    let mut input = String::new();

    // Read first input
    stdin.read_line(&mut input).expect("Failed to read line");
    a = input.trim().parse().expect("Invalid input for a");

    // Read second input
    input.clear();
    stdin.read_line(&mut input).expect("Failed to read line");
    b = input.trim().parse().expect("Invalid input for b");

    // Read third input
    input.clear();
    stdin.read_line(&mut input).expect("Failed to read line");
    c = input.trim().parse().expect("Invalid input for c");

    // Calculate the difference and the product
    let difference = c - (a + b);
    let product = a * b;

    // Check the condition: if the difference is greater than the product
    if difference > product {
        println!("Yes");
    } else {
        println!("No");
    }

    // Return 0 to indicate successful execution
    std::process::exit(0);
}
