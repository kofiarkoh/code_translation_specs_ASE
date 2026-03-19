use std::io;

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as an integer
    let n: i32 = input.trim().parse().expect("Input must be a valid integer");

    // Calculate the quotient and remainder when n is divided by 11
    let quotient = n / 11; // Integer division
    let remainder = n % 11; // Remainder when n is divided by 11

    // Compute the output based on the specified formula
    // The output should be non-negative, so we ensure it is calculated correctly
    let output = (quotient * 2) + remainder;

    // Output the result
    println!("{}", output);
}
