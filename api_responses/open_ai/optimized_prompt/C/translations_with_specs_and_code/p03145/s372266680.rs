use std::io::{self, Write};

fn main() {
    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input string into parts and parse them as integers
    let mut parts = input.split_whitespace();
    let a: i32 = parts.next().expect("Expected first integer").parse().expect("Invalid integer");
    let b: i32 = parts.next().expect("Expected second integer").parse().expect("Invalid integer");
    // The third integer is read but not used
    let _c: i32 = parts.next().expect("Expected third integer").parse().expect("Invalid integer");

    // Calculate half the product of the first two integers
    let ans = 0.5 * (a as f64 * b as f64);
    
    // Print the result formatted as an integer
    // Using `round()` to ensure proper rounding behavior
    println!("{:.0}", ans.round());
}
