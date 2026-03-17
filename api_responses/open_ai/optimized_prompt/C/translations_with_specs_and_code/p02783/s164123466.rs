use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read two integers from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    let a: i32 = iter.next().expect("Expected first integer").parse().expect("Invalid integer");
    let b: i32 = iter.next().expect("Expected second integer").parse().expect("Invalid integer");

    // Preconditions: b must not be zero
    if b == 0 {
        panic!("Division by zero is undefined");
    }

    // Calculate the ceiling of the division
    let mut c = a / b;
    if a % b != 0 {
        c += 1; // Increment if there's a remainder
    }

    // Output the result followed by a newline
    println!("{}", c);
}
