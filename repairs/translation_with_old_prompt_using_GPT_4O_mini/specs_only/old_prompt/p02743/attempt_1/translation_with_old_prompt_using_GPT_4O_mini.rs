use std::io::{self, Write};

fn main() {
    // Declare variables for the inputs
    let mut a: f64 = 0.0; // Using f64 to represent long double
    let mut b: f64 = 0.0;
    let mut c: f64 = 0.0;

    // Read inputs from standard input
    // Note: In Rust, we typically use `std::io::stdin()` for input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the input into three f64 values
    let values: Vec<&str> = input.split_whitespace().collect();
    if values.len() < 3 {
        // If there are not enough inputs, we cannot proceed
        eprintln!("Not enough input values.");
        return;
    }
    
    // Attempt to parse the inputs; if parsing fails, we assume invalid input
    a = values[0].parse().unwrap_or(0.0);
    b = values[1].parse().unwrap_or(0.0);
    c = values[2].parse().unwrap_or(0.0);

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
