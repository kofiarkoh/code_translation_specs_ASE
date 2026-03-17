use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read three integers from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    // Parse the first two integers
    let a: i32 = iter.next().unwrap().parse().expect("Invalid input for a");
    let b: i32 = iter.next().unwrap().parse().expect("Invalid input for b");
    
    // The third integer is read but not used
    let _c: i32 = iter.next().unwrap().parse().expect("Invalid input for c");
    
    // Calculate half the product of a and b
    let ans = (a * b) as f64 * 0.5;
    
    // Print the result rounded to the nearest whole number
    // Using `round()` to ensure it follows the specification for rounding
    println!("{:.0}", ans.round());
}
