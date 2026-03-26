use std::io::{self, Write};

fn main() {
    // Read input values for x and y
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    // Parse x and y as integers
    let x: i32 = iter.next().unwrap().parse().expect("Invalid input for x");
    let y: i32 = iter.next().unwrap().parse().expect("Invalid input for y");

    // Initialize result variable
    let mut result: i64;

    // Conditional logic based on the relationship between x and y
    if x > y {
        result = (x - y) as i64; // If x is greater than y, compute x - y
    } else if x < y {
        result = (y - x) as i64; // If x is less than y, compute y - x
    } else {
        result = 0; // If x equals y, the result is 0
    }

    // Print the result to standard output
    println!("{}", result);
}
