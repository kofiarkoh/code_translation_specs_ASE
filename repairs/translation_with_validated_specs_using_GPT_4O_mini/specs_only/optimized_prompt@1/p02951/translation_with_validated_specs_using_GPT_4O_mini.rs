use std::io::{self, Write};

fn main() {
    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into parts and parse them as integers
    let mut parts = input.split_whitespace();
    let a: i32 = parts.next().expect("Expected first integer").parse().expect("Invalid integer");
    let b: i32 = parts.next().expect("Expected second integer").parse().expect("Invalid integer");
    let mut c: i32 = parts.next().expect("Expected third integer").parse().expect("Invalid integer");

    // Calculate the modified first integer based on the difference of the first two
    let modified_a = (a - b).max(0); // Ensure modified_a is non-negative

    // Decrement c based on the modified_a while both are positive
    while modified_a > 0 && c > 0 {
        c -= 1; // Decrement c
    }

    // Output the final value of c
    println!("{}", c);
}
