use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as a signed 64-bit integer
    let x: i64 = input.trim().parse().expect("Failed to parse input");

    // Calculate the output based on the conditions
    let result = if x % 11 == 0 {
        2 * (x / 11)
    } else if x % 11 > 6 {
        2 * (x / 11) + 2
    } else {
        2 * (x / 11) + 1
    };

    // Print the result
    println!("{}", result);
}
